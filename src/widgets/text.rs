use std::{cell::RefCell, rc::Rc};

use crate::{ widgets::{helpers::default_style::DefaultStyle, props::dirty::Dirty}, Color, DefStyle, Font, GBuf, Style, Widget};


pub struct Text {
    buf: GBuf,
    dirty: Dirty,
    font: Rc<Font>,
    size: f32,
    color: (u8, u8, u8),
    style: Rc<Style>
}

impl Text {
    
    pub fn new(str: &str, style: Option<Rc<Style>>) -> Rc<RefCell<Self>> {

        let style = DefaultStyle::optional_style::<Self>(style);
        let font = style.get_font().unwrap_or(Font::default());
        let size = style.get_font_size().unwrap_or(24.0);

        let [r,g,b,_] = Color::u32_to_rgba(
            style.get_color().unwrap_or(0)
        );

        let buf = font.text(str, size, (r, g, b));

        Rc::new(RefCell::new(Self {
            buf,
            dirty: Dirty { is_dirty: false },
            font,
            size,
            color: (r, g, b),
            style
        }))
    }

    pub fn change_text(&mut self, str: &str){
        self.buf = self.font.text(str, self.size, self.color);
        self.dirty.mark();
    }

}

impl Widget for Text {

    fn force_build(&mut self, _: (usize, usize)) {
        
    }
    
    fn size(&self) -> (usize, usize) {
        self.buf.size()
    }

    fn flush(&mut self) {

    }

    fn is_dirty(&self) -> bool {
        self.dirty.check()
    }
    fn update_dirty_state(&mut self, _: (usize, usize)) {
    }
    fn update(&mut self, _: (usize, usize)) {
        
    }
    fn style(&self) -> Rc<Style> {
        self.style.clone()
    }
    fn draw(&mut self, _: (usize, usize)) -> (&Vec<u32>, usize, usize) {
        self.dirty.clear();
        let (w, h) = self.buf.size();
        (self.buf.read(), w, h)
    }
}

impl DefStyle for Text {
    fn default_style() -> Style {
        Style::new().font(Font::default()).font_size(24.0).color(Color::rgba_to_u32(0, 0, 0, 255))
    }
}