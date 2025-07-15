
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {

    #[inline(always)]
    pub fn rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
        (r as u32) << 24 |
        (g as u32) << 16 |
        (b as u32) << 8 |
        (a as u32)
    }

    #[inline(always)]
    pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
        (r as u32) << 16 |
        (g as u32) << 8 |
        (b as u32)
    }

    #[inline(always)]
    pub fn u32_to_rgba(c: u32) -> [u8; 4]{
        [
            (c >> 24) as u8,
            (c >> 16) as u8,
            (c >> 8) as u8,
            c as u8
        ]
    }

    #[inline(always)]
    pub fn u32_to_rgb(c: u32) -> [u8; 3]{
        [
            (c >> 16) as u8,
            (c >> 8) as u8,
            c as u8
        ]
    }

    pub fn from_rgba(c: u32) -> Self {
        let c = Self::u32_to_rgba(c);
        Self(c[0], c[1], c[2], c[3])
    }

    pub fn from_rgb(c: u32) -> Self {
        let c = Self::u32_to_rgb(c);
        Self(c[0], c[1], c[2], 255)
    }

    pub fn rgba(&self) -> u32 {
        Self::rgba_to_u32(self.0, self.1, self.2, self.3)
    }

    pub fn rgb(&self) -> u32 {
        u32::from_le_bytes([0, self.0, self.1, self.2])
    }
}