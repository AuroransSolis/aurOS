pub enum VgaColour {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGrey
}

impl VgaColour {
    fn value(&self) -> u16 {
        match self {
            VgaColour::Black => 0b0000,
            VgaColour::Blue => 0b0001,
            VgaColour::Green => 0b0010,
            VgaColour::Cyan => 0b0011,
            VgaColour::Red => 0b0100,
            VgaColour::Magenta => 0b0101,
            VgaColour::Brown => 0b0110,
            VgaColour::LightGrey => 0b0111,
        }
    }

    fn pack_into_foreground(&self, bright: bool, n: &mut u16) {
        let complete_val = (bright as u16) << 3 | self.value();
        *n |= complete_val << 7;
    }

    fn pack_into_background(&self, bright: bool, n: &mut u16) {
        let complete_val = (bright as u16) << 3 | self.value();
        *n |= complete_val << 11;
    }
}

pub struct VgaChar {
    cp437_char: u8,
    foreground_colour: VgaColour,
    bright_foreground: bool,
    background_colour: VgaColour,
    bright_background: bool,
}