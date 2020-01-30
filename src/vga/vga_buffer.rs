use super::vga_char::{VgaChar, VgaColour};
use volatile::Volatile;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
struct VgaBuffer {
    chars: [[Volatile<VgaChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaWriter {
    column_position: usize,
    colour_code: u8,
    buf: &'static mut VgaBuffer,
}

impl VgaWriter {
    pub fn write_byte(&mut self, b: u8) {
        match b {
            b'\n' => self.write_line(),
            _ => {
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let colour_code = self.colour_code;
                self.buf.chars[row][col].write(VgaChar::new(b, colour_code));
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                // 0x20(space)..0x7e(pipe) or newline
                b' '..=b'|' | b'\n' => self.write_byte(b),
                // Everything else is outside the printable ASCII range, write ■ as a filler
                _ => self.write_byte(0xfe)
            }
            self.advance_cursor();
        }
    }

    pub fn write_line(&mut self) {

    }

    pub fn advance_column(&mut self) {
        self.column_position += 1;
    }

    pub fn set_colour_code(&mut self, fg: VgaColour, bg: VgaColour) {
        self.colour_code = VgaColour::packed_fg_bg(fg, bg);
    }

    pub fn advance_cursor(&mut self) {
        self.column_position += 1;
        if self.column_position == BUFFER_WIDTH {
            self.column_position = 0;
            self.write_line();
        }
    }
}

pub fn vga_test() {
    let mut writer = VgaWriter {
        column_position: 0,
        colour_code: VgaColour::packed_fg_bg(VgaColour::Red, VgaColour::White),
        buf: unsafe { &mut *(0xb8000 as *mut VgaBuffer) }
    };
    writer.write_string("Hello");
    writer.set_colour_code(VgaColour::White, VgaColour::Red);
    writer.write_string(" world!");
}