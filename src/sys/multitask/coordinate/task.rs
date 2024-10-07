use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::task::{Context, Poll};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub(super) struct TaskId(usize);

impl TaskId {
    fn new() -> TaskId {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task {
    pub id: TaskId,
    future: Pin<Box<dyn Future<Output=()>>>,
}

impl Task {
    pub fn new(future: impl Future<Output=()> + 'static) -> Task {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    pub(super) fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}