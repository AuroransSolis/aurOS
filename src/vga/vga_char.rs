// Makeup of a VGA screen character:
// bits 0-7: character codepoint (specifically from code page 437, but basically ASCII)
// bits 8-11: foreground colour - most significant bit is the "bright bit"
// bits 12-14: background colour - most significant bit is the "bright bit"
// bit 15: blinking cursor

// VGA colours and their representation as a u8. Four bits would technically be enough, but a byte
// is the smallest integer size in Rust.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum VgaColour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGrey = 7,
    DarkGrey = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

impl VgaColour {
    pub const fn packed_fg_bg(fg: Self, bg: Self) -> u8 {
        (fg as u8) | ((bg as u8) << 4)
    }
}

#[derive(Copy, Clone)]
pub struct VgaChar {
    cp437_char: u8,
    colour_code: u8,
}

impl VgaChar {
    pub fn new(cp437_char: u8, colour_code: u8) -> Self {
        VgaChar {
            cp437_char,
            colour_code,
        }
    }
}

pub const BLANK_CHAR: VgaChar = VgaChar {
    cp437_char: b' ',
    colour_code: 0b00000000,
};
