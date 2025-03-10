mod frame_allocator;

//=================================================
// Constants
//=================================================

pub const PAGE_SIZE: usize = 4096;

//=================================================
// Structs
//=================================================

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize
}

impl Frame {
    fn containing_address(address: usize) -> Self {
        Frame { number: address / PAGE_SIZE }
    }
}

pub trait FrameAllocator { 
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

