use crate::widgets::Widget;


pub struct Nil {
    buf: Vec<u32>
}

impl Nil {
    pub fn new() -> Self {
        Self { buf: vec![0] }
    }
}

impl Widget for Nil {
    fn size(&self) -> (usize, usize) {
        (0, 0)
    }
    fn update(&mut self, _: (usize, usize)) {
    }
    fn draw(&mut self, _: (usize, usize)) -> &Vec<u32> {
        &self.buf
    }
    fn flush(&mut self) {
        
    }
    fn is_dirty(&self) -> bool {
        false
    }
}