use std::rc::Rc;

use fontdue::Font as F;

use crate::{Color, GBuf};


pub struct Font {
    font: F
}

impl Font {
    
    pub fn default() -> Rc<Self> {
        const FONT_DATA: &[u8] = include_bytes!("../../src/fonts/def.ttf");
        let font = F::from_bytes(FONT_DATA, fontdue::FontSettings::default()).unwrap();

        Rc::new(Self {
            font,
        })
    }

    pub fn rasterize_char_u32(&self, ch: char, size: f32, (r, g, b): (u8, u8, u8)) -> (usize, usize,Vec<u32>) {
        let (metrics, bitmap) = self.font.rasterize(ch, size);
        let mut pix = Vec::with_capacity(metrics.width * metrics.height);

        for j in 0..metrics.height {
            for i in 0..metrics.width {
                let alpha = bitmap[j * metrics.width + i];
                let px: u32 = Color::rgba_to_u32(r, g, b, alpha);
                pix.push(px);
            }
        }
        (metrics.width, metrics.height,pix)
    }


    pub fn text(&self, s: &str, size: f32, color: (u8, u8, u8)) -> GBuf {

        let spacing = (size * 0.08) as usize;
        let space_char_size = (size * 0.15) as usize;

        let mut chars = Vec::new();
        let (mut w, mut h) = (0, 0);

        for c in s.chars() {
            if c == ' ' {
                w += space_char_size + spacing;
                chars.push((space_char_size, 0, vec![]));
                continue;
            }
            let (cw, ch, c_buf) = self.rasterize_char_u32(c, size, color);
            w += cw + spacing;
            h = if h < ch { ch } else { h };
            chars.push((cw, ch, c_buf));
        }

        let mut buf = GBuf::new(w, h , 0);

        let mut current_x = 0;
        for (cw, ch, c_buf) in chars.iter() {

        

            buf.merge(current_x, h - ch + 1,(c_buf, cw.to_owned(), ch.to_owned()));
            current_x += cw + spacing;
        }
        buf
    }
}