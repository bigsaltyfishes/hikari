use crossbeam_utils::atomic::AtomicCell;

pub struct Cell<T> {
    inner: AtomicCell<Option<T>>,
}

impl<T> Default for Cell<T> {
    fn default() -> Self {
        Self {
            inner: AtomicCell::default()
        }
    }
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: AtomicCell::new(Some(value))
        }
    }

    pub fn init(&self, value: T) {
        self.inner.store(Some(value));
    }

    pub const fn uninit() -> Self {
        Self {
            inner: AtomicCell::new(None)
        }
    }

    pub fn is_init(&self) -> bool {
        let inner = self.inner.take();
        let is_some = inner.is_some();
        self.inner.store(inner);
        is_some
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        unsafe { self.inner.as_ptr().as_mut().unwrap().as_mut().unwrap() }
    }

    pub unsafe fn as_raw_ptr(&self) -> (usize, usize) {
        let inner = self.inner.as_ptr().as_mut().unwrap().as_mut().unwrap();
        (inner as *const _ as usize, core::mem::size_of_val(inner))
    }
}