
//=================================================
// Imports
//=================================================

use crate::{print, println};
use core::fmt;
use spin::Mutex;

//=================================================
// Constants
//=================================================

const VGA_BUFFER: *mut ScreenChar = 0xb8000 as *mut ScreenChar;

// Represents the VGA buffer's actual size.
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

//=================================================
// Static
//=================================================

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
});

//=================================================
// Structs
//=================================================

#[allow(unused)]
#[derive(Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> Self {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    ascii_char: u8,
    color_code: ColorCode
}

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
}

unsafe impl<'a> Send for Writer {}
unsafe impl<'a> Sync for Writer {}

impl<'a> Writer {

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                
                unsafe {
                    *VGA_BUFFER.add(row * BUFFER_WIDTH + col) = ScreenChar {
                        ascii_char: byte,
                        color_code: self.color_code,
                    }
                }

                self.column_position += 1;
            }
        }
    }
    
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
               unsafe {
                let c: ScreenChar = *VGA_BUFFER.add(row * BUFFER_WIDTH + col);
                *VGA_BUFFER.add((row - 1) * BUFFER_WIDTH + col) = c;
               }
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_char: b' ',
            color_code: self.color_code
        };
        for col in 0..BUFFER_WIDTH {
            unsafe {
                *VGA_BUFFER.add(row * BUFFER_WIDTH + col) = blank;
            }
        }
    }

}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          self.write_byte(byte)
        }
        Ok(())
    }
}

//=================================================
// Enums
//=================================================

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

//=================================================
// Standalone Functions
//=================================================

pub fn clear_screen() {
    for _ in 0..BUFFER_HEIGHT {
        println!("");
    }
}
