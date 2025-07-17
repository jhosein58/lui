use crate::{widgets::{props::{dirty::Dirty, style::Style}, Widget}, GBuf, Size};


pub struct Rectangle {
    buf: GBuf,
    dirty: Dirty,
    w: Size,
    h: Size
}

impl Rectangle {
    
    pub fn new(style: Style) -> Self {

        let w = style.get_width().unwrap_or(Size::Absolute(0));
        let h = style.get_height().unwrap_or(Size::Absolute(0));

        let (buf_w, buf_h);

        if let Size::Absolute(w) = w {
            buf_w = w;
        }else {
            buf_w = 0;
        }
        
        if let Size::Absolute(h) = w {
            buf_h = h;
        }else {
            buf_h = 0;
        }
    
        Self { 
            buf: GBuf::new(
            buf_w, 
            buf_h, 
            style.get_color().unwrap_or(0)
        ) , 
        dirty: Dirty { is_dirty: true },
        w,
        h
    }

    }
}

impl Widget for Rectangle {

    fn size(&self) -> (usize, usize) {
        self.buf.size()
    }

    fn update(&mut self, (par_w, par_h): (usize, usize)) {
        
        let new_w = match self.w {
            Size::Relative(w) => (w * par_w as f32) as usize,
            Size::Absolute(w) => w
        };

        let new_h = match self.h {
            Size::Relative(h) => (h * par_h as f32) as usize,
            Size::Absolute(h) => h
        };

        self.buf.resize_if_needed(new_w, new_h);
        
        self.dirty.clear();
    }
    fn draw(&mut self, par_size: (usize, usize)) -> &Vec<u32> {
        self.update(par_size);
        self.buf.read()
    }

    fn flush(&mut self) {
        self.buf.flush();
    }
    fn is_dirty(&self) -> bool {
        self.dirty.check()
    }
}