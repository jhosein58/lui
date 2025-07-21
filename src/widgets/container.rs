

use std::rc::Rc;

use crate::{ widgets::{helpers::default_style::DefaultStyle, props::dirty::Dirty}, BorderRadius, DefStyle, GBuf, Size, Style, Widget};


pub struct Container {
    buf: GBuf,
    children: Vec<Box<dyn Widget>>,
    w: Size,
    h: Size,
    br: usize,
    dirty: Dirty,
    last_build: Option<(usize, usize)>,
    initialized: bool,
    style: Rc<Style>
}

impl Container {
    
    pub fn new(children: Vec<Box<dyn Widget>>, style: Option<Rc<Style>>) -> Box<Self> {

        let style = DefaultStyle::optional_style::<Self>(style);

        let w = style.get_width().unwrap_or(Size::Relative(1.0));
        let h = style.get_height().unwrap_or(Size::Relative(1.0));
        let br = style.get_border_radius().unwrap_or(0);

        let (buf_w, buf_h);

        if let Size::Absolute(w_val) = w {
            buf_w = w_val;
        }else {
            buf_w = 0;
        }
        
        if let Size::Absolute(h_val) = h {
            buf_h = h_val;
        }else {
            buf_h = 0;
        }
    
        Box::new(Self { 
            children,
            buf: GBuf::new(buf_w, buf_h, style.get_color().unwrap_or(0),) , 
            dirty: Dirty { is_dirty: true },
            w,
            h,
            br,
            last_build: None,
            initialized: false,
            style
        })
    }

    fn init_build_if_need(&mut self, par_size: (usize, usize)) {

        if !self.initialized {
            self.force_build(par_size);
            self.initialized = true;
        }

    }

}

impl Widget for Container {

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

        self.buf.resize_if_needed(new_w, new_h);
        self.flush();


        
        // let mut wrapper = Wrapper {
        //         layout: ColumnLayout {
        //             spacing: 5
        //         },
        //         children: &mut self.children
        //     };

        // wrapper.render(&mut self.buf);

        self.buf.process(BorderRadius { radius: self.br });
        self.dirty.clear();
        self.last_build = Some((new_w, new_h));
    }

    fn size(&self) -> (usize, usize) {
        self.buf.size()
    }

    fn flush(&mut self) {
        self.buf.flush();
        self.dirty.mark();
    }

    fn is_dirty(&self) -> bool {

        if self.dirty.check() {
            return true;
        }
        for child in self.children.iter(){
            if child.is_dirty() {
                return true;
            }
        }

        false  
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

       for child in self.children.iter_mut() {
            child.update_dirty_state(par_size);
        }
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

    fn style(&self) -> Rc<Style> {
        self.style.clone()
    }
}

impl DefStyle for Container {
    fn default_style() -> Style {
        Style::new().color(0).width(Size::Relative(1.0)).height(Size::Relative(1.0)).border_radius(0)
    }
}