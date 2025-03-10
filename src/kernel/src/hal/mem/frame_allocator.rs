use crate::hal;
use hal::mem::{Frame, FrameAllocator};
use multiboot2::MemoryArea;

pub struct AreaFrameAllocator {
    next_free_frame: Frame,
    current_area: Option<&'static MemoryArea>,
    area: MemoryArea
}
