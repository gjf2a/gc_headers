#![no_std]

pub trait GarbageCollectingHeap {
    fn new() -> Self;
    fn load(&self, p: Pointer) -> anyhow::Result<u64, HeapError>;
    fn store(&mut self, p: Pointer, value: u64) -> anyhow::Result<(), HeapError>;
    fn malloc<T: Tracer>(&mut self, num_words: usize, tracer: &T) -> anyhow::Result<Pointer, HeapError>;
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
    pub fn new(block: usize, size: usize) -> Self {
        Self {block, offset: 0, size}
    }

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

#[derive(Debug, thiserror::Error)]
pub enum HeapError {
    #[error("Out of blocks")]
    OutOfBlocks,
    #[error("Out of memory")]
    OutOfMemory,
    #[error("Illegal block: {0}")]
    IllegalBlock(usize),
    #[error("Offset too large: {0}")]
    OffsetTooBig(usize),
}