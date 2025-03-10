//=================================================
// Imports
//=================================================

use crate::hal;
use hal::mem::{Frame, FrameAllocator};
use multiboot2::MemoryArea;

//=================================================
// Structs
//=================================================

pub struct AreaFrameAllocator<'a> {
    next_free_frame: Frame,
    current_area: Option<&'a MemoryArea>,
    areas:  &'a [MemoryArea],
    kernel_start: Frame,
    kernel_end: Frame,
    multiboot_start: Frame,
    multiboot_end: Frame
}

impl <'a> FrameAllocator for AreaFrameAllocator<'a> {
    fn allocate_frame(&mut self) -> Option<Frame> {
        loop {
            if let Some(area) = self.current_area {
                let frame: Frame = Frame { number: self.next_free_frame.number };
    
                let current_area_last_frame: Frame = {
                    let address= area.start_address() + area.size() - 1;
                    Frame::containing_address(address as usize)
                };
    
                if frame > current_area_last_frame {
                    // All frames in the current area are used, switch to the next area
                    self.choose_next_area();
                    if self.current_area.is_none() {
                        return None;
                    }
                } else if frame >= self.multiboot_start && frame <= self.multiboot_end {
                    // Skip frames occupied by Multiboot
                    self.next_free_frame = Frame { 
                        number: self.multiboot_end.number + 1
                    };
                } else {
                    // Valid frame found
                    self.next_free_frame.number += 1;
                    return Some(frame);
                }
            } else {
                return None; // No available frames
            }
        }
    }

    fn deallocate_frame(&mut self, frame: Frame) {
        unimplemented!()
    }
}

impl <'a> AreaFrameAllocator<'a> {
    /// # Description
    /// Instantiates a new `AreaFrameAllocator`.
    pub fn new(
        kernel_start: usize,
        kernel_end: usize,
        multiboot_start: usize,
        multiboot_end: usize,
        memory_areas: &'a [MemoryArea]
    ) -> Self {
        let mut allocator = AreaFrameAllocator {
            next_free_frame: Frame::containing_address(0),
            current_area: None,
            areas: memory_areas,
            kernel_start: Frame::containing_address(kernel_start),
            kernel_end: Frame::containing_address(kernel_end),
            multiboot_start: Frame::containing_address(multiboot_start),
            multiboot_end: Frame::containing_address(multiboot_end),
        };
        allocator.choose_next_area();
        allocator
    }
    
    /// # Description
    /// Filters out already exhausted memory areas
    /// and picks the lowest starting address among
    /// remaining areas, avoiding skipping availiable
    /// frames.
    fn choose_next_area(&mut self) {
        self.current_area = self.areas
            .iter()
            .filter(|area| {
                let addr = area.start_address() + area.size() - 1;
                Frame::containing_address(addr as usize) >= self.next_free_frame
            })
            .min_by_key(|area| area.start_address());

        if let Some(area) = self.current_area {
            let start_frame: Frame = Frame::containing_address(area.start_address() as usize);
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }   
    }
}