use super::vga_char::VgaChar;

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]
struct VgaBuffer {
    chars: [[VgaChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
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
                self.buf[row][col] = VgaChar::new(b, colour_code);
            }
        }
    }

    pub fn set_colour_code(&mut self, colour_code: u8) {
        self.colour_code = colour_code;
    }

    pub fn advance_cursor(&mut self) {
        if self.column_position == BUFFER_WIDTH {
            self.new_line()
        }
    }

    pub fn write_line(&mut self, ) {

    }
}