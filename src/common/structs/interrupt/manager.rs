use crate::abstracts::interrupt::controller::IrqHandler;
use core::ops::Range;
use id_alloc::IdAlloc;

#[derive(Debug)]
pub enum IrqError {
    InvalidIrqVector,
    FailedToAllocIrqVector,
    HandlerAlreadyRegistered,
    HandlerNotRegistered,
}

pub type IrqResult<T = ()> = Result<T, IrqError>;

pub struct IrqManager<const IRQ_NUM: usize> {
    irq_idx_base: usize,
    handlers: [Option<IrqHandler>; IRQ_NUM],
    allocator: IdAlloc,
}

impl<const IRQ_NUM: usize> IrqManager<IRQ_NUM> {
    pub fn new(vec_range: Range<usize>) -> IrqManager<IRQ_NUM> {
        const EMPTY: Option<IrqHandler> = None;
        Self {
            irq_idx_base: vec_range.start,
            handlers: [EMPTY; IRQ_NUM],
            allocator: IdAlloc::with_capacity(vec_range.len()),
        }
    }

    pub fn register_handler(&mut self, vector: usize, handler: IrqHandler) -> IrqResult<usize> {
        let idx = if vector == 0 {
            self.allocator.alloc().ok_or(IrqError::FailedToAllocIrqVector)?
        } else {
            let irq_idx = vector.checked_sub(self.irq_idx_base)
                .ok_or(IrqError::InvalidIrqVector)?;
            self.allocator.alloc_specific(irq_idx)
                .ok_or(IrqError::FailedToAllocIrqVector)?
        };
        self.handlers[idx] = Some(handler);
        Ok(idx)
    }

    pub fn unregister_handler(&mut self, vector: usize) -> IrqResult {
        let idx = vector - self.irq_idx_base;
        if idx >= IRQ_NUM {
            return Err(IrqError::InvalidIrqVector);
        }
        if self.handlers[idx].is_none() {
            return Err(IrqError::HandlerNotRegistered);
        }
        self.handlers[idx] = None;
        self.allocator.free(idx);
        Ok(())
    }

    pub fn overwrite_handler(&mut self, vector: usize, handler: IrqHandler) -> IrqResult {
        let idx = vector - self.irq_idx_base;
        if idx >= IRQ_NUM {
            return Err(IrqError::InvalidIrqVector);
        }
        if self.handlers[idx].is_some() {
            return Err(IrqError::HandlerAlreadyRegistered);
        }
        self.handlers[idx] = Some(handler);
        Ok(())
    }

    pub fn handle_irq(&self, vector: usize) -> IrqResult {
        let idx = vector - self.irq_idx_base;
        if idx >= IRQ_NUM {
            return Err(IrqError::InvalidIrqVector);
        }
        if let Some(handler) = &self.handlers[idx] {
            handler(vector);
            Ok(())
        } else {
            Err(IrqError::HandlerNotRegistered)
        }
    }
}