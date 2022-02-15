//! Common color structures used in vga programming.

/// Represents the size of the vga palette in bytes.
pub const PALETTE_SIZE: usize = 768;

/// Represents a 16 bit color used for vga display.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color16 {
    /// Represents the color `Black (0x0)`.
    Black = 0x0,
    /// Represents the color `Blue (0x1)`.
    Blue = 0x1,
    /// Represents the color `Green (0x2)`.
    Green = 0x2,
    /// Represents the color `Cyan (0x3)`.
    Cyan = 0x3,
    /// Represents the color `Red (0x4)`.
    Red = 0x4,
    /// Represents the color `Magenta (0x5)`.
    Magenta = 0x5,
    /// Represents the color `Brown (0x6)`.
    Brown = 0x6,
    /// Represents the color `LightGrey (0x7)`.
    LightGrey = 0x7,
    /// Represents the color `DarkGrey (0x8)`.
    DarkGrey = 0x8,
    /// Represents the color `LightBlue (0x9)`.
    LightBlue = 0x9,
    /// Represents the color `LightGreen (0xA)`.
    LightGreen = 0xA,
    /// Represents the color `LightCyan (0xB)`.
    LightCyan = 0xB,
    /// Represents the color `LightRed (0xC)`.
    LightRed = 0xC,
    /// Represents the color `Pink (0xD)`.
    Pink = 0xD,
    /// Represents the color `Yellow (0xE)`.
    Yellow = 0xE,
    /// Represents the color `White (0xF)`.
    White = 0xF,
}

impl From<Color16> for u8 {
    fn from(value: Color16) -> u8 {
        value as u8
    }
}

/// Represents a color for vga text modes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct TextModeColor(u8);

impl TextModeColor {
    /// Returns a new `TextModeColor` given the specified `foreground`
    /// and `background` color.
    pub const fn new(foreground: Color16, background: Color16) -> TextModeColor {
        TextModeColor((background as u8) << 4 | (foreground as u8))
    }

    /// Sets the background color given the specified `background`;
    pub fn set_background(&mut self, background: Color16) {
        self.0 = (background as u8) << 4 | (self.0 & 0x0F);
    }

    /// Sets the foreground color given the specified `foreground`.
    pub fn set_foreground(&mut self, foreground: Color16) {
        self.0 = foreground as u8;
    }
}

/// Represents the default vga 256 color palette.
pub const DEFAULT_PALETTE: [u8; PALETTE_SIZE] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x2a, 0x00, 0x2a, 0x00, 0x00, 0x2a, 0x2a, 0x2a, 0x00, 0x00, 0x2a,
    0x00, 0x2a, 0x2a, 0x15, 0x00, 0x2a, 0x2a, 0x2a, 0x15, 0x15, 0x15, 0x15, 0x15, 0x3f, 0x15, 0x3f,
    0x15, 0x15, 0x3f, 0x3f, 0x3f, 0x15, 0x15, 0x3f, 0x15, 0x3f, 0x3f, 0x3f, 0x15, 0x3f, 0x3f, 0x3f,
    0x00, 0x00, 0x00, 0x05, 0x05, 0x05, 0x08, 0x08, 0x08, 0x0b, 0x0b, 0x0b, 0x0e, 0x0e, 0x0e, 0x11,
    0x11, 0x11, 0x14, 0x14, 0x14, 0x18, 0x18, 0x18, 0x1c, 0x1c, 0x1c, 0x20, 0x20, 0x20, 0x24, 0x24,
    0x24, 0x28, 0x28, 0x28, 0x2d, 0x2d, 0x2d, 0x32, 0x32, 0x32, 0x38, 0x38, 0x38, 0x3f, 0x3f, 0x3f,
    0x00, 0x00, 0x3f, 0x10, 0x00, 0x3f, 0x1f, 0x00, 0x3f, 0x2f, 0x00, 0x3f, 0x3f, 0x00, 0x3f, 0x3f,
    0x00, 0x2f, 0x3f, 0x00, 0x1f, 0x3f, 0x00, 0x10, 0x3f, 0x00, 0x00, 0x3f, 0x10, 0x00, 0x3f, 0x1f,
    0x00, 0x3f, 0x2f, 0x00, 0x3f, 0x3f, 0x00, 0x2f, 0x3f, 0x00, 0x1f, 0x3f, 0x00, 0x10, 0x3f, 0x00,
    0x00, 0x3f, 0x00, 0x00, 0x3f, 0x10, 0x00, 0x3f, 0x1f, 0x00, 0x3f, 0x2f, 0x00, 0x3f, 0x3f, 0x00,
    0x2f, 0x3f, 0x00, 0x1f, 0x3f, 0x00, 0x10, 0x3f, 0x1f, 0x1f, 0x3f, 0x27, 0x1f, 0x3f, 0x2f, 0x1f,
    0x3f, 0x37, 0x1f, 0x3f, 0x3f, 0x1f, 0x3f, 0x3f, 0x1f, 0x37, 0x3f, 0x1f, 0x2f, 0x3f, 0x1f, 0x27,
    0x3f, 0x1f, 0x1f, 0x3f, 0x27, 0x1f, 0x3f, 0x2f, 0x1f, 0x3f, 0x37, 0x1f, 0x3f, 0x3f, 0x1f, 0x37,
    0x3f, 0x1f, 0x2f, 0x3f, 0x1f, 0x27, 0x3f, 0x1f, 0x1f, 0x3f, 0x1f, 0x1f, 0x3f, 0x27, 0x1f, 0x3f,
    0x2f, 0x1f, 0x3f, 0x37, 0x1f, 0x3f, 0x3f, 0x1f, 0x37, 0x3f, 0x1f, 0x2f, 0x3f, 0x1f, 0x27, 0x3f,
    0x2d, 0x2d, 0x3f, 0x31, 0x2d, 0x3f, 0x36, 0x2d, 0x3f, 0x3a, 0x2d, 0x3f, 0x3f, 0x2d, 0x3f, 0x3f,
    0x2d, 0x3a, 0x3f, 0x2d, 0x36, 0x3f, 0x2d, 0x31, 0x3f, 0x2d, 0x2d, 0x3f, 0x31, 0x2d, 0x3f, 0x36,
    0x2d, 0x3f, 0x3a, 0x2d, 0x3f, 0x3f, 0x2d, 0x3a, 0x3f, 0x2d, 0x36, 0x3f, 0x2d, 0x31, 0x3f, 0x2d,
    0x2d, 0x3f, 0x2d, 0x2d, 0x3f, 0x31, 0x2d, 0x3f, 0x36, 0x2d, 0x3f, 0x3a, 0x2d, 0x3f, 0x3f, 0x2d,
    0x3a, 0x3f, 0x2d, 0x36, 0x3f, 0x2d, 0x31, 0x3f, 0x00, 0x00, 0x1c, 0x07, 0x00, 0x1c, 0x0e, 0x00,
    0x1c, 0x15, 0x00, 0x1c, 0x1c, 0x00, 0x1c, 0x1c, 0x00, 0x15, 0x1c, 0x00, 0x0e, 0x1c, 0x00, 0x07,
    0x1c, 0x00, 0x00, 0x1c, 0x07, 0x00, 0x1c, 0x0e, 0x00, 0x1c, 0x15, 0x00, 0x1c, 0x1c, 0x00, 0x15,
    0x1c, 0x00, 0x0e, 0x1c, 0x00, 0x07, 0x1c, 0x00, 0x00, 0x1c, 0x00, 0x00, 0x1c, 0x07, 0x00, 0x1c,
    0x0e, 0x00, 0x1c, 0x15, 0x00, 0x1c, 0x1c, 0x00, 0x15, 0x1c, 0x00, 0x0e, 0x1c, 0x00, 0x07, 0x1c,
    0x0e, 0x0e, 0x1c, 0x11, 0x0e, 0x1c, 0x15, 0x0e, 0x1c, 0x18, 0x0e, 0x1c, 0x1c, 0x0e, 0x1c, 0x1c,
    0x0e, 0x18, 0x1c, 0x0e, 0x15, 0x1c, 0x0e, 0x11, 0x1c, 0x0e, 0x0e, 0x1c, 0x11, 0x0e, 0x1c, 0x15,
    0x0e, 0x1c, 0x18, 0x0e, 0x1c, 0x1c, 0x0e, 0x18, 0x1c, 0x0e, 0x15, 0x1c, 0x0e, 0x11, 0x1c, 0x0e,
    0x0e, 0x1c, 0x0e, 0x0e, 0x1c, 0x11, 0x0e, 0x1c, 0x15, 0x0e, 0x1c, 0x18, 0x0e, 0x1c, 0x1c, 0x0e,
    0x18, 0x1c, 0x0e, 0x15, 0x1c, 0x0e, 0x11, 0x1c, 0x14, 0x14, 0x1c, 0x16, 0x14, 0x1c, 0x18, 0x14,
    0x1c, 0x1a, 0x14, 0x1c, 0x1c, 0x14, 0x1c, 0x1c, 0x14, 0x1a, 0x1c, 0x14, 0x18, 0x1c, 0x14, 0x16,
    0x1c, 0x14, 0x14, 0x1c, 0x16, 0x14, 0x1c, 0x18, 0x14, 0x1c, 0x1a, 0x14, 0x1c, 0x1c, 0x14, 0x1a,
    0x1c, 0x14, 0x18, 0x1c, 0x14, 0x16, 0x1c, 0x14, 0x14, 0x1c, 0x14, 0x14, 0x1c, 0x16, 0x14, 0x1c,
    0x18, 0x14, 0x1c, 0x1a, 0x14, 0x1c, 0x1c, 0x14, 0x1a, 0x1c, 0x14, 0x18, 0x1c, 0x14, 0x16, 0x1c,
    0x00, 0x00, 0x10, 0x04, 0x00, 0x10, 0x08, 0x00, 0x10, 0x0c, 0x00, 0x10, 0x10, 0x00, 0x10, 0x10,
    0x00, 0x0c, 0x10, 0x00, 0x08, 0x10, 0x00, 0x04, 0x10, 0x00, 0x00, 0x10, 0x04, 0x00, 0x10, 0x08,
    0x00, 0x10, 0x0c, 0x00, 0x10, 0x10, 0x00, 0x0c, 0x10, 0x00, 0x08, 0x10, 0x00, 0x04, 0x10, 0x00,
    0x00, 0x10, 0x00, 0x00, 0x10, 0x04, 0x00, 0x10, 0x08, 0x00, 0x10, 0x0c, 0x00, 0x10, 0x10, 0x00,
    0x0c, 0x10, 0x00, 0x08, 0x10, 0x00, 0x04, 0x10, 0x08, 0x08, 0x10, 0x0a, 0x08, 0x10, 0x0c, 0x08,
    0x10, 0x0e, 0x08, 0x10, 0x10, 0x08, 0x10, 0x10, 0x08, 0x0e, 0x10, 0x08, 0x0c, 0x10, 0x08, 0x0a,
    0x10, 0x08, 0x08, 0x10, 0x0a, 0x08, 0x10, 0x0c, 0x08, 0x10, 0x0e, 0x08, 0x10, 0x10, 0x08, 0x0e,
    0x10, 0x08, 0x0c, 0x10, 0x08, 0x0a, 0x10, 0x08, 0x08, 0x10, 0x08, 0x08, 0x10, 0x0a, 0x08, 0x10,
    0x0c, 0x08, 0x10, 0x0e, 0x08, 0x10, 0x10, 0x08, 0x0e, 0x10, 0x08, 0x0c, 0x10, 0x08, 0x0a, 0x10,
    0x0b, 0x0b, 0x10, 0x0c, 0x0b, 0x10, 0x0d, 0x0b, 0x10, 0x0f, 0x0b, 0x10, 0x10, 0x0b, 0x10, 0x10,
    0x0b, 0x0f, 0x10, 0x0b, 0x0d, 0x10, 0x0b, 0x0c, 0x10, 0x0b, 0x0b, 0x10, 0x0c, 0x0b, 0x10, 0x0d,
    0x0b, 0x10, 0x0f, 0x0b, 0x10, 0x10, 0x0b, 0x0f, 0x10, 0x0b, 0x0d, 0x10, 0x0b, 0x0c, 0x10, 0x0b,
    0x0b, 0x10, 0x0b, 0x0b, 0x10, 0x0c, 0x0b, 0x10, 0x0d, 0x0b, 0x10, 0x0f, 0x0b, 0x10, 0x10, 0x0b,
    0x0f, 0x10, 0x0b, 0x0d, 0x10, 0x0b, 0x0c, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_foreground() {
        let mut color = TextModeColor::new(Color16::Yellow, Color16::Black);
        color.set_foreground(Color16::Red);
        assert_eq!(color.0 & 0x0F, Color16::Red as u8);
    }

    #[test]
    fn test_set_background() {
        let mut color = TextModeColor::new(Color16::Yellow, Color16::Black);
        color.set_background(Color16::DarkGrey);
        assert_eq!(color.0 >> 4, Color16::DarkGrey as u8);
    }
}
