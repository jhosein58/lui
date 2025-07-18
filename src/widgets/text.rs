use crate::{ widgets::props::dirty::Dirty, Color, Font, GBuf, Widget};


pub struct Text<'a> {
    buf: GBuf,
    font: &'a Font,
    dirty: Dirty
}

impl<'a> Text<'a> {
    
    pub fn new(str: &str, size: f32, color: Color, font: &'a Font) -> Box<Self> {
        let buf = font.text(str, size, (color.0, color.1, color.2));

        Box::new(Self {
            buf,
            font,
            dirty: Dirty { is_dirty: false }
        })
    }

}

impl<'a> Widget for Text<'a> {
    
    fn size(&self) -> (usize, usize) {
        self.buf.size()
    }

    fn flush(&mut self) {
        self.buf.flush();
        self.bu
    }
}