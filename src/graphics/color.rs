
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {

    #[inline(always)]
    pub fn rgba_to_u32(r: u8, g: u8, b: u8, a: u8) -> u32 {
        (a as u32) << 24 |
        (r as u32) << 16 |
        (g as u32) << 8 |
        (b as u32)
    }

    #[inline(always)]
    pub fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
        Self::rgba_to_u32(r, g, b, 0)
    }

    #[inline(always)]
    pub fn u32_to_rgba(c: u32) -> [u8; 4]{
        [
            (c >> 16) as u8,
            (c >> 8) as u8,
            c as u8,
            (c >> 24) as u8
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

    #[inline(always)]
    fn u8_to_float(c: u8) -> f32{
        c as f32 / 255.0
    }

    #[inline(always)]
    fn float_to_u8(c: f32) -> u8{
        (c * 255.0).round() as u8
    }

    #[inline(always)]
    fn blend_alpha(sa: u8, da: u8) -> u8{
        let sa = Self::u8_to_float(sa);
        let da = Self::u8_to_float(da);
        Self::float_to_u8(sa + da * (1.0 - sa))
    }

    #[inline(always)]
    fn blend_color_segment(sc: u8, sa: u8, dc: u8, da: u8, blended_alpha: u8) -> u8{
        
        let sc: f32 = Self::u8_to_float(sc);
        let dc: f32 = Self::u8_to_float(dc);
        let sa: f32 = Self::u8_to_float(sa);
        let da: f32 = Self::u8_to_float(da);
        let blended_alpha: f32 = Self::u8_to_float(blended_alpha);
        
        Self::float_to_u8((sc * sa + dc * da * (1.0 - sa)) / blended_alpha)
    }


    #[inline(always)]
    pub fn  blend_to_rgba(src: [u8; 4], dst: [u8; 4]) -> [u8; 4]{

        let out_a: u8 = Self::blend_alpha(src[3], dst[3]);
        let out_r: u8 = Self::blend_color_segment(src[0], src[3], dst[0], dst[3], out_a);
        let out_g: u8 = Self::blend_color_segment(src[1], src[3], dst[1], dst[3], out_a);
        let out_b: u8 = Self::blend_color_segment(src[2], src[3], dst[2], dst[3], out_a);

        [out_r, out_g, out_b, out_a]
    }

    #[inline(always)]
    pub fn blend_to_u32(src: [u8; 4], dst: [u8; 4]) -> u32 {
        let [r, g, b, a] = Self::blend_to_rgba(src, dst);
        Self::rgba_to_u32(r, g, b, a)
    }

    #[inline(always)]
    pub fn blend_u32_to_u32(src: u32, dst: u32) -> u32 {
        let (src, dst) = (Self::u32_to_rgba(src), Self::u32_to_rgba(dst));
        let [r, g, b, a] = Self::blend_to_rgba(src, dst);
        Self::rgba_to_u32(r, g, b, a)
    }

    pub fn  from_blend(src: Self, dst: Self) -> Self{

        let [out_r, out_g, out_b, out_a] = Self::blend_to_rgba([src.0,src.1,src.2,src.3], [dst.0,dst.1,dst.2,dst.3]);
        Self(out_r, out_g, out_b, out_a)
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
         Self::rgb_to_u32(self.0, self.1, self.2)
    }
}