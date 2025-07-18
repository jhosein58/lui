use crate::{widgets::Widget, Style};


pub struct Nil {
    buf: Vec<u32>
}

impl Nil {
    pub fn new() -> Box<Self> {
        Box::new(Self { buf: vec![] })
    }
}

impl Widget for Nil {
    fn size(&self) -> (usize, usize) {
        (0, 0)
    }
    fn update(&mut self, _: (usize, usize)) {
    }
    fn draw(&mut self, _: (usize, usize)) -> (usize, usize, &Vec<u32>) {
    (0,0,&self.buf)
    }
    fn flush(&mut self) {
        
    }
    fn is_dirty(&self) -> bool {
        false
    }
    fn update_dirty_state(&mut self, _: (usize, usize)) {
        
    }
    fn style(&self) -> Style {
        Style::new()
    }
}