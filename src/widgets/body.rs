use crate::{widgets::Widget, Color, GBuf};


pub struct Body<T> where T: Widget {
    buf: GBuf,
    children: Vec<T>
} 

impl<T> Body<T> where T: Widget {

    pub fn new(w: usize, h: usize, children: Vec<T>) -> Self {
        Self { 
            buf: GBuf::new(w, h, Color::rgb_to_u32(255, 255, 255)),
            children
        }
    
    }
}

impl<T> Widget for Body<T> where T: Widget {

    fn size(&self) -> (usize, usize) {
        self.buf.size()
    }

    fn update(&mut self, (par_w, par_h): (usize, usize)) {
        self.buf.resize_if_needed(par_w, par_h);


        for child in self.children.iter_mut() {
            let (w, h) = child.size();
            self.buf.merge(10, 10, child.draw((par_w,par_h)), w, h).unwrap();
        }
    }
    fn draw(&mut self, par_size: (usize, usize)) -> &Vec<u32> {
        self.update(par_size);
        self.buf.read()
    }

    fn flush(&mut self) {
        self.buf.flush();
    }

    fn is_dirty(&self) -> bool {
        false
    }
}