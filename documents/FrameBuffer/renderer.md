# Code snapshot
path: src/library/renderer/mod.rs

```rust
// src/library/renderer/mod.rs
use bootloader::BootInfo;

pub use self::text_renderer::TEXTWRITER;

pub mod color;
pub mod text_renderer;

pub fn init(boot_info: &'static mut BootInfo) {
    text_renderer::init(boot_info);
}
```

```rust
// src/library/renderer/color.rs
#[repr(C)]
pub struct ColorRGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

pub enum Color {
    Black       =  1,
    Blue        =  2,
    Green       =  3,
    Cyan        =  4,
    Red         =  5,
    Magenta     =  6,
    Brown       =  7,
    LightGray   =  8,
    Gray        =  9,
    LightBlue   = 10,
    LightGreen  = 11,
    LightCyan   = 12,
    LightRed    = 13,
    Pink        = 14,
    Yellow      = 15,
    White       = 16,
}

impl ColorRGB {
    pub fn vga(color: Color) -> Self {
        match color {
            Color::Black        => ColorRGB {red: 0x00, green: 0x00, blue: 0x00 },
            Color::Blue         => ColorRGB {red: 0x00, green: 0x02, blue: 0xaa },
            Color::Green        => ColorRGB {red: 0x14, green: 0xaa, blue: 0x00 },
            Color::Cyan         => ColorRGB {red: 0x00, green: 0xaa, blue: 0xaa },
            Color::Red          => ColorRGB {red: 0xaa, green: 0x00, blue: 0x03 },
            Color::Magenta      => ColorRGB {red: 0xaa, green: 0x00, blue: 0xaa },
            Color::Brown        => ColorRGB {red: 0xaa, green: 0x55, blue: 0x00 },
            Color::LightGray    => ColorRGB {red: 0xaa, green: 0xaa, blue: 0xaa },
            Color::Gray         => ColorRGB {red: 0x55, green: 0x55, blue: 0x55 },
            Color::LightBlue    => ColorRGB {red: 0x55, green: 0x55, blue: 0xff },
            Color::LightGreen   => ColorRGB {red: 0x55, green: 0xff, blue: 0x55 },
            Color::LightCyan    => ColorRGB {red: 0x55, green: 0xff, blue: 0xff },
            Color::LightRed     => ColorRGB {red: 0xff, green: 0x55, blue: 0x55 },
            Color::Pink         => ColorRGB {red: 0xfd, green: 0x55, blue: 0xff },
            Color::Yellow       => ColorRGB {red: 0xff, green: 0xff, blue: 0x55 },
            Color::White        => ColorRGB {red: 0xff, green: 0xff, blue: 0xff },
        }
    }
}
```

```rust
// src/library/renderer/text_renderer.rs
use super::color::{Color, ColorRGB};

use core::fmt::{Arguments, Write};

use bootloader::{
    boot_info::{FrameBufferInfo, PixelFormat},
    BootInfo,
};
use noto_sans_mono_bitmap::{get_bitmap, get_bitmap_width, BitmapChar, BitmapHeight, FontWeight};

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
        let writer = TEXTWRITER.get_or_init(move || {
            Mutex::new(TextWriter::new(
                framebuffer,
                info,
                ColorRGB::vga(Color::Yellow),
                ColorRGB::vga(Color::Black),
            ))
        });
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
    foreground_color: ColorRGB,
    background_color: ColorRGB,
}

impl TextWriter {
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo, foreground_color: ColorRGB, background_color: ColorRGB) -> Self {
        let mut writer = Self {
            framebuffer,
            info,
            x_position: 0,
            y_position: 0,
            foreground_color,
            background_color,
        };
        writer.clear();
        writer
    }

    pub fn set_foreground_color(&mut self, color: ColorRGB) {
        self.foreground_color = color;
    }

    pub fn set_background_color(&mut self, color: ColorRGB) {
        self.background_color = color;
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
        self.y_position = self.height() - CURSOR_HEIGHT;
    }

    pub fn shift_frame(&mut self, lines: usize) {
        for _ in 0..lines {
            // get each bitmap line has pixel numbers
            let nums = self.bytes_per_text_line();
            let total = self.info.byte_len;

            for i in 0..total - nums {
                self.framebuffer[i] = self.framebuffer[i + nums];
            }
            // clean remain
            for i in total - nums..total {
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
                if self.x_position >= self.width() {
                    self.newline();
                }

                const BITMAP_LETTER_WIDTH: usize =
                    get_bitmap_width(FontWeight::Regular, BitmapHeight::Size16);
                if self.y_position >= (self.height() - BITMAP_LETTER_WIDTH) {
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
                self.write_pixel(self.x_position + x, self.y_position + y, *byte);
            }
        }
        self.x_position += rendered_char.width();
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        fn color_convert<'a>(intensity: u8, foreground_color: &'a ColorRGB, background_color: &'a ColorRGB) -> ColorRGB {
            // ColorRGB { 
            //     red: (intensity as i32 * foreground_color.red as i32 / 255) as u8, 
            //     green: (intensity as i32 * foreground_color.green as i32 / 255) as u8, 
            //     blue: (intensity as i32 * foreground_color.blue as i32 / 255) as u8,  
            // }

            // assume intensity below and equal 75 is background
    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        fn color_convert<'a>(intensity: u8, foreground_color: &'a ColorRGB, background_color: &'a ColorRGB) -> ColorRGB {
            // ColorRGB { 
            //     red: (intensity as i32 * foreground_color.red as i32 / 255) as u8, 
            //     green: (intensity as i32 * foreground_color.green as i32 / 255) as u8, 
            //     blue: (intensity as i32 * foreground_color.blue as i32 / 255) as u8,  
            // }

            // assume intensity below and equal 75 is background
            match intensity {
                0..=50 => ColorRGB { 
                    red: background_color.red, 
                    green: background_color.green,
                    blue: background_color.blue
                },
                // 51..=100 => ColorRGB { 
                //     red: (intensity as i32 * background_color.red as i32 / 255) as u8, 
                //     green: (intensity as i32 * background_color.green as i32 / 255) as u8, 
                //     blue: (intensity as i32 * background_color.blue as i32 / 255) as u8,  
                // },
                // _ => ColorRGB { 
                //     red: (intensity as i32 * foreground_color.red as i32 / 255) as u8, 
                //     green: (intensity as i32 * foreground_color.green as i32 / 255) as u8, 
                //     blue: (intensity as i32 * foreground_color.blue as i32 / 255) as u8,  
                // }
                _ => ColorRGB { 
                    red: foreground_color.red,
                    green: foreground_color.green,
                    blue: foreground_color.blue,
                }
            }
        }

        let pixel_offset = y * self.info.stride + x;
        let color_rgb = color_convert(intensity, &self.foreground_color, &self.background_color);
        let color = match self.info.pixel_format {
            PixelFormat::RGB => [color_rgb.red, color_rgb.green, color_rgb.blue, 0],
            PixelFormat::BGR => [color_rgb.blue, color_rgb.green, color_rgb.red, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            _ => [0, 0, 0, 0], // make rust-analyzer pass
        };
        let bytes_pre_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_pre_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_pre_pixel)]
            .copy_from_slice(&color[..bytes_pre_pixel]);
        let _ = unsafe { core::ptr::read_volatile(&self.framebuffer[byte_offset]) };
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
        $crate::library::renderer::text_renderer::_print(format_args!($($arg)*));
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

```