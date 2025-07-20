use std::rc::Rc;

use crate::{widgets::{helpers::default_style::DefaultStyle, props::{dirty::Dirty, style::Style}, Widget}, BorderRadius, DefStyle, GBuf, Size};


pub struct Rectangle {
    buf: GBuf,
    dirty: Dirty,
    w: Size,
    h: Size,
    br: usize,
    last_build: Option<(usize, usize)>,
    initialized: bool,
    style: Rc<Style>
}

impl Rectangle {
    
    pub fn new(style: Option<Rc<Style>>) -> Box<Self> {

        let style = DefaultStyle::optional_style::<Self>(style);

        let w = style.get_width().unwrap_or(Size::Absolute(0));
        let h = style.get_height().unwrap_or(Size::Absolute(0));
        let br = style.get_border_radius().unwrap_or(0);

    
        Box::new(Self { 
            buf: GBuf::new(
            0, 
            0, 
            style.get_color().unwrap_or(0),
        ) , 
        dirty: Dirty { is_dirty: true },
        w,
        h,
        br,
        last_build: None,
        initialized: false,
        style
    })

    }

    fn force_build(&mut self, par_size: (usize, usize)) {

        
        let new_w = match self.w {
            Size::Relative(w) => {
                (w * par_size.0 as f32) as usize
            },
            Size::Absolute(w) => w
        };

        let new_h = match self.h {
            Size::Relative(h) => (h * par_size.1 as f32) as usize,
            Size::Absolute(h) => h
        };

        self.buf.resize(new_w, new_h);
        self.buf.process(BorderRadius{ radius: self.br });
        self.dirty.clear();
        self.last_build = Some((new_w, new_h));
    }

    fn init_build_if_need(&mut self, par_size: (usize, usize)) {

        if !self.initialized {
            self.force_build(par_size);
            self.initialized = true;
        }

    }

}

impl Widget for Rectangle {

    fn size(&self) -> (usize, usize) {
        self.buf.size()
    }

    fn update(&mut self, par_size: (usize, usize)) {
        
        self.update_dirty_state(par_size);

        if self.is_dirty() {
           self.force_build(par_size);
        }
        
    }


    
    fn draw(&mut self, par_size: (usize, usize)) -> (&Vec<u32>, usize, usize) {
        
        self.update(par_size);
        let (w, h) = self.buf.size();
        (self.buf.read(), w, h)
    }

    fn flush(&mut self) {
        self.buf.flush();
        self.dirty.mark();
    }
    fn is_dirty(&self) -> bool {
        self.dirty.check()
    }
    fn update_dirty_state(&mut self, par_size: (usize, usize)) {
        self.init_build_if_need(par_size);

        let w_old = self.last_build.unwrap_or((0,0)).0;

        let w_need_update = if let Size::Relative(w_val) = self.w {
            w_old != ( w_val * par_size.0 as f32 ) as usize
        }else {
            false
        };
    

        let h_old = self.last_build.unwrap_or((0,0)).1;
        let h_need_update = if let Size::Relative(h_val) = self.h {
            h_old != ( h_val * par_size.1 as f32 ) as usize
        }else {
            false
        };


       if w_need_update || h_need_update{
        self.dirty.mark();
       }else {
           self.dirty.clear();
       }
    }

    fn style(&self) -> Rc<Style> {
        self.style.clone()
    }

}

impl DefStyle for Rectangle {
    fn default_style() -> Style {
        Style::new().color(0).width(Size::Absolute(0)).height(Size::Absolute(0)).border_radius(0)
    }
}