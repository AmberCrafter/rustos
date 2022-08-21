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