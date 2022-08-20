use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

use super::{ColorCode, screen::{Buffer, ScreenChar}, VGA_BUFFER_WIDTH, VGA_BUFFER_HEIGHT, VGA_BUFFER_ADDR, Color};

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGreen, Color::Black),
        buffer: unsafe {
            &mut *(VGA_BUFFER_ADDR as *mut Buffer)
        }
    });
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position>=VGA_BUFFER_WIDTH {
                    self.new_line()
                }
                let row = VGA_BUFFER_HEIGHT-1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar::new(
                    byte, color_code
                ));
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..VGA_BUFFER_HEIGHT {
            for col in 0..VGA_BUFFER_WIDTH {
                let char = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(char)
            }
        }
        self.clear_row(VGA_BUFFER_HEIGHT-1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar::new(b' ', self.color_code);
        for col in 0..VGA_BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank)
        }
    }
}


impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}