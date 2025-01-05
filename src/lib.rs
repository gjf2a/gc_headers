#![no_std]
use thiserror_no_std::Error;

pub trait GarbageCollectingHeap {
    fn new() -> Self;
    fn load(&self, p: Pointer) -> anyhow::Result<u64, HeapError>;
    fn store(&mut self, p: Pointer, value: u64) -> anyhow::Result<(), HeapError>;
    fn malloc<T: Tracer>(&mut self, num_words: usize, tracer: &T) -> anyhow::Result<Pointer, HeapError>;
    fn blocks_num_copies(&self) -> impl Iterator<Item = (usize, usize)>;
    fn is_allocated(&self, block: usize) -> bool;
    fn num_allocated_blocks(&self) -> usize {
        self.blocks_num_copies().count()
    }
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

#[derive(Clone, Copy, Eq, PartialEq, Debug, Error)]
pub enum HeapError {
    #[error("No more blocks in heap")]
    OutOfBlocks,
    #[error("No blocks available of the requested size")]
    OutOfMemory,
    #[error("Invalid block number")]
    IllegalBlock,
    #[error("Invalid offset into block")]
    OffsetTooBig,
}

impl core::error::Error for HeapError {}