use core::fmt::{Arguments, Write};

use bootloader::{boot_info::{FrameBufferInfo, PixelFormat}, BootInfo};
use noto_sans_mono_bitmap::{get_bitmap_width, FontWeight, BitmapHeight, get_bitmap, BitmapChar};

use conquer_once::spin::OnceCell;
use spin::Mutex;
const CURSOR_HEIGHT: usize = BitmapHeight::Size16.val();
const LINE_SPACING: usize = 0;

pub static TEXTWRITER: OnceCell<Mutex<TextWriter>> = OnceCell::uninit();


pub fn init(boot_info: &'static mut BootInfo) {
    // init TEXTWRITER
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info();
        let framebuffer = framebuffer.buffer_mut();
        let writer = TEXTWRITER.get_or_init(move || Mutex::new(
            TextWriter { framebuffer, info, x_position: 0, y_position: 0 }
        ));
        writer.lock().clear();
    } else {
        panic!("TEXTWRITER initialize failed");
    }
}


pub struct TextWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_position: usize,
    y_position: usize,
}

impl TextWriter {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut writer = Self { framebuffer, info, x_position: 0, y_position: 0 };
        writer.clear();
        writer
    }

    pub fn clear(&mut self) {
        self.x_position = 0;
        self.y_position = 0;
        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.horizontal_resolution
    }

    fn height(&self) -> usize {
        self.info.vertical_resolution
    }

    fn carriage_return(&mut self) {
        self.x_position = 0;
    }

    fn newline(&mut self) {
        self.y_position += CURSOR_HEIGHT + LINE_SPACING;
        self.carriage_return();
    }

    fn bytes_per_text_line(&self) -> usize {
        self.info.bytes_per_pixel * self.info.stride * CURSOR_HEIGHT
    }

    pub fn cursor_last_line(&mut self) {
        self.x_position = 0;
        self.y_position = self.info.byte_len - self.bytes_per_text_line();
    }

    pub fn shift_frame(&mut self, lines: usize) {
        for _ in 0..lines {
            // get each bitmap line has pixel numbers
            let nums = bytes_per_text_line();
            let total = self.info.byte_len;
    
            for i in 0..total-nums {
                self.framebuffer[i] = self.framebuffer[i+nums];
    
            }
            // clean remain
            for i in total-nums..total {
                self.framebuffer[i] = 0x0;
            }
        }
        unsafe {
            core::ptr::read_volatile(&self.framebuffer);
        }
    }

    pub fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                if self.x_position>=self.width() {
                    self.newline();
                }
                
                const BITMAP_LETTER_WIDTH: usize = get_bitmap_width(FontWeight::Regular, BitmapHeight::Size16);
                if self.y_position >= (self.height()-BITMAP_LETTER_WIDTH) {
                    self.clear();
                }
                let bitmap_char = get_bitmap(c, FontWeight::Regular, BitmapHeight::Size16).unwrap();
                self.write_rendered_char(bitmap_char);
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: BitmapChar) {
        for (y, row) in rendered_char.bitmap().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_position + x , self.y_position + y, *byte);
            }
        }
        self.x_position += rendered_char.width();
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y*self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::RGB => [intensity, intensity, intensity/2, 0],
            PixelFormat::BGR => [intensity/2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity>200 {0xf} else {0}, 0, 0, 0],
            _ => [0,0,0,0],                 // make rust-analyzer pass
        };
        let bytes_pre_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_pre_pixel;
        self.framebuffer[byte_offset..(byte_offset+bytes_pre_pixel)]
            .copy_from_slice(&color[..bytes_pre_pixel]);
        let _ = unsafe {
            core::ptr::read_volatile(&self.framebuffer[byte_offset])
        };

    }
}

unsafe impl Send for TextWriter {}
unsafe impl Sync for TextWriter {}

impl core::fmt::Write for TextWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}


// setup global interface
#[doc(hidden)]
pub fn _print(args: Arguments) {
    if let Some(writer) = TEXTWRITER.get() {
        writer
            .lock()
            .write_fmt(args)
            .expect("Printing to render failed");
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::library::render::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => {
        ($crate::print!(
            concat!($fmt, "\n"), $($arg)*
        ));
    };
}