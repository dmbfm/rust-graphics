#[allow(dead_code)]

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Color::new(r, g, b, 255)
    }

    pub fn pack(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.b as u32) << 16) | ((self.g as u32) << 8) | (self.r as u32)
    }

    pub fn unpack(pixel: u32) -> Color {
        Color::new(
            (pixel & 255) as u8,
            ((pixel >> 8) & 255) as u8,
            ((pixel >> 16) & 255) as u8,
            ((pixel >> 24) & 255) as u8,
        )
    }
}
