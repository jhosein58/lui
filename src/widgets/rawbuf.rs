use std::rc::Rc;

use crate::{ Style, Widget};


pub struct RawBuf {
    buf: Vec<u32>,
    w: usize, 
    h: usize
}

impl RawBuf {
    pub fn new(buf: Vec<u32>, w: usize, h: usize) -> Box<Self> {
        Box::new(
            Self { buf, w, h }
        )
    }
}

impl Widget for RawBuf {
    fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }
    fn flush(&mut self) {
        
    }
    fn is_dirty(&self) -> bool {
        false
    }
    fn style(&self) -> Rc<Style> {
        Style::new().get().unwrap()
    }
    fn update_dirty_state(&mut self, _: (usize, usize)) {
        
    }
    fn update(&mut self, _: (usize, usize)) {
        
    }
    fn draw(&mut self, _: (usize, usize)) -> (&Vec<u32>, usize, usize) {
        let (w, h) = self.size();
        (&self.buf, w, h)
    }
}