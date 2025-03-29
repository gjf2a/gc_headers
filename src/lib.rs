#![cfg_attr(not(test), no_std)]
use thiserror_no_std::Error;

pub trait GarbageCollectingHeap {
    fn new() -> Self;
    fn load(&self, p: Pointer) -> anyhow::Result<u64, HeapError>;
    fn store(&mut self, p: Pointer, value: u64) -> anyhow::Result<(), HeapError>;
    fn address(&self, p: Pointer) -> anyhow::Result<usize, HeapError>;
    fn blocks_in_use(&self) -> impl Iterator<Item = usize>;
    fn allocated_block_ptr(&self, block: usize) -> Option<Pointer>;
    fn blocks_num_copies(&self) -> impl Iterator<Item = (usize, usize)>;
    
    fn malloc<T: Tracer>(
        &mut self,
        num_words: usize,
        tracer: &T,
    ) -> anyhow::Result<Pointer, HeapError>;

    fn num_allocated_blocks(&self) -> usize {
        self.blocks_in_use().count()
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
        Self {
            block,
            offset: 0,
            size,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Pointer> {
        PointerIter { next_ptr: *self }
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
}

struct PointerIter {
    next_ptr: Pointer,
}

impl Iterator for PointerIter {
    type Item = Pointer;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_ptr.offset < self.next_ptr.size {
            let result = Some(self.next_ptr);
            self.next_ptr.offset += 1;
            result
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
    #[error("Block number {0} out of range; max block number is {1}")]
    IllegalBlock(usize, usize),
    #[error("Block not allocated: {0}")]
    UnallocatedBlock(usize),
    #[error("Invalid offset ({0}) into block {1}; max offset is {2}")]
    OffsetTooBig(usize, usize, usize),
    #[error("Invalid heap address {0}; max address is {1}")]
    IllegalAddress(usize, usize),
}

impl core::error::Error for HeapError {}

#[cfg(test)]
mod tests {
    use crate::Pointer;

    #[test]
    fn test_pointer_iteration() {
        let p = Pointer::new(0, 5);
        let addresses = p.iter().collect::<Vec<_>>();
        assert_eq!(5, addresses.len());
        for i in 0..5 {
            assert_eq!(i, addresses[i].offset());
        }
    }
}
