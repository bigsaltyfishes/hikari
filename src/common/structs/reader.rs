use core::cell::Cell;

pub trait Reader {
    fn as_ptr(&self) -> *const u8;
    fn as_slice(&self) -> &[u8];
    fn read<T>(&self) -> Option<T>;
    fn read_u8(&self) -> Option<u8>;
    fn read_u16(&self) -> Option<u16>;
    fn read_u32(&self) -> Option<u32>;
    fn read_u64(&self) -> Option<usize>;
}

#[derive(Debug, Clone)]
pub struct DataReader {
    data: Cell<*const u8>,
    limit: Cell<*const u8>,
}

impl DataReader {
    pub fn new(data: *const u8, limit: *const u8) -> Self {
        Self {
            data: Cell::new(data),
            limit: Cell::new(limit),
        }
    }

    pub fn reset_limit(&self, limit: *const u8) {
        self.limit.set(limit);
    }

    pub fn len(&self) -> usize {
        self.limit.get() as usize - self.data.get() as usize
    }

    pub fn offset_forward(&self, offset: usize) -> Option<Self> {
        if self.data.get() as usize + offset >= self.limit.get() as usize {
            return None;
        }
        Some(Self {
            data: Cell::new(unsafe { self.data.get().add(offset) }),
            limit: self.limit.clone(),
        })
    }
}

impl Reader for DataReader {
    fn as_ptr(&self) -> *const u8 {
        self.data.get()
    }

    fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.data.get(), self.limit.get() as usize - self.data.get() as usize) }
    }

    fn read<T>(&self) -> Option<T> {
        let value_size = core::mem::size_of::<T>();
        let data = self.data.get();
        if self.len() >= value_size {
            let result = unsafe { (data as *const T).read_unaligned() };
            unsafe { self.data.swap(&Cell::new(data.add(value_size))) };
            return Some(result);
        };
        None
    }

    fn read_u8(&self) -> Option<u8> {
        self.read()
    }

    fn read_u16(&self) -> Option<u16> {
        self.read()
    }

    fn read_u32(&self) -> Option<u32> {
        self.read()
    }

    fn read_u64(&self) -> Option<usize> {
        if core::mem::size_of::<usize>() < 8 {
            None
        } else {
            self.read()
        }
    }
}