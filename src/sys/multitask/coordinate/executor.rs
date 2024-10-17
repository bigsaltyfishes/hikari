use crate::sys::multitask::coordinate::task::{Task, TaskId};
use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use alloc::task::Wake;
use core::task::{Context, Waker};
use crossbeam_queue::ArrayQueue;

const QUEUE_CAPACITY: usize = 100;

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker { task_id, task_queue }))
    }

    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}

pub struct Executor {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(QUEUE_CAPACITY)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let task_id = task.id;
        if self.tasks.insert(task_id, task).is_some() {
            panic!("Task with same ID already in tasks");
        }
        self.task_queue.push(task_id).expect("queue full");
    }

    pub fn run(&mut self) -> ! {
        loop {
            self.run_ready_tasks();
        }
    }

    pub fn sleep_if_idle(&self) {
        // TODO: Implement this after interrupt subsystem is implemented
        unimplemented!()
    }

    fn run_ready_tasks(&mut self) {
        let Self { tasks, task_queue, waker_cache } = self;

        while let Some(task_id) = task_queue.pop() {
            let task = match tasks.get_mut(&task_id) {
                Some(task) => task,
                None => continue
            };

            let waker = waker_cache
                .entry(task_id)
                .or_insert_with(|| TaskWaker::new(task_id, task_queue.clone()));

            let mut context = Context::from_waker(waker);
            match task.poll(&mut context) {
                core::task::Poll::Pending => {}
                core::task::Poll::Ready(()) => { tasks.remove(&task_id); }
            }
        }
    }
}