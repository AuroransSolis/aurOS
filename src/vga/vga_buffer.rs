use super::vga_char::{VgaChar, VgaColour, BLANK_CHAR};
use core::fmt;
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
    // Using a pointer instead of a mutable reference since it makes sense to use one for memory
    // that's out of our control
    buf: *mut VgaBuffer,
}

impl VgaWriter {
    pub fn write_byte(&mut self, b: u8) {
        match b {
            b'\n' => self.new_line(),
            _ => {
                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let colour_code = self.colour_code;
                unsafe { &mut *self.buf }.chars[row][col].write(VgaChar::new(b, colour_code));
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                // 0x20(space)..0x7e(pipe) or newline
                b' '..=b'|' | b'\n' => self.write_byte(b),
                // Everything else is outside the printable ASCII range, write ■ as a filler
                _ => self.write_byte(0xfe),
            }
            self.advance_cursor();
        }
    }

    pub fn new_line(&mut self) {
        for r in 1..BUFFER_HEIGHT {
            for c in 0..BUFFER_WIDTH {
                let copy = unsafe { &*self.buf }.chars[r][c].read();
                unsafe { &mut *self.buf }.chars[r - c][c].write(copy);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    pub fn clear_row(&mut self, row: usize) {
        for c in 0..BUFFER_WIDTH {
            unsafe { &mut *self.buf }.chars[row][c].write(BLANK_CHAR);
        }
    }

    pub fn set_fg_bg(&mut self, fg: VgaColour, bg: VgaColour) {
        self.colour_code = VgaColour::packed_fg_bg(fg, bg);
    }

    pub fn set_fg(&mut self, fg: VgaColour) {
        // Set the lower four bits (foreground bits) to 0
        self.colour_code &= 0b11110000;
        // Apply the new foreground code
        self.colour_code |= fg as u8;
    }

    pub fn set_bg(&mut self, bg: VgaColour) {
        // Set the upper four bits (background bits) to 0
        self.colour_code &= 0b00001111;
        // Apply the new background code
        self.colour_code |= (bg as u8) << 4;
    }

    pub fn advance_cursor(&mut self) {
        self.column_position += 1;
        if self.column_position == BUFFER_WIDTH {
            self.column_position = 0;
            self.new_line();
        }
    }
}

// Technically safe since only one VGA writer is constructed, and it should have unique access to
// the VGA buffer pointer. Necessary for use in `spin::Mutex`.
unsafe impl Send for VgaWriter {}

// Global VGA writer - `spin::Mutex` is used for synchronized interior mutability.
pub static VGA_WRITER: spin::Mutex<VgaWriter> = spin::Mutex::new(VgaWriter {
    column_position: 0,
    colour_code: VgaColour::packed_fg_bg(VgaColour::Red, VgaColour::White),
    buf: 0xb8000 as *mut VgaBuffer,
});

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        Ok(self.write_string(s))
    }
}
