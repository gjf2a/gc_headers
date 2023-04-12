#![no_std]

pub trait GarbageCollectingHeap {
    fn new() -> Self;
    fn load(&self, p: Pointer) -> HeapResult<u64>;
    fn store(&mut self, p: Pointer, value: u64) -> HeapResult<()>;
    fn malloc<T: Tracer>(&mut self, num_words: usize, tracer: &T) -> HeapResult<Pointer>;
}

pub trait Tracer {
    fn trace(&self, blocks_used: &mut [bool]);
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Debug)]
pub struct Pointer {
    block: usize,
    offset: usize,
    size: usize,
}

impl Pointer {
    pub fn len(&self) -> usize {
        self.size
    }

    pub fn block_num(&self) -> usize {
        self.block
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn next(&self) -> Option<Self> {
        let offset = self.offset + 1;
        if offset < self.size {
            Some(Self {
                block: self.block,
                offset,
                size: self.size,
            })
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum HeapResult<T> {
    Ok(T),
    Err(HeapError),
}

impl<T> HeapResult<T> {
    pub fn unwrap(self) -> T {
        match self {
            HeapResult::Ok(value) => value,
            HeapResult::Err(e) => panic!("Heap Error: {e:?}"),
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> HeapResult<U> {
        match self {
            HeapResult::Ok(value) => HeapResult::Ok(op(value)),
            HeapResult::Err(e) => HeapResult::Err(e),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum HeapError {
    OutOfBlocks,
    OutOfMemory,
    IllegalBlock,
    OffsetTooBig,
}