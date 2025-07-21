
use std::{cell::RefCell, rc::Rc};

use crate::{widgets::{helpers::default_style::DefaultStyle, props::dirty::Dirty, DefStyle, Widget}, Color, GBuf, PositionLayout, Screen, Style, Wrapper};


pub struct Body {
    buf: GBuf,
    children: Rc<Vec<Rc<RefCell<dyn Widget>>>>,
    dirty: Dirty,
    last_build: Option<(usize, usize)>,
    style: Rc<Style>
} 

impl Body {

    pub fn new(children: Rc<Vec<Rc<RefCell<dyn Widget>>>>, style: Option<Rc<Style>>) -> Self {

        let accepted_style = DefaultStyle::optional_style::<Self>(style);

        Self { 
            buf: GBuf::new(0, 0, accepted_style.most_have_color(0xFFFFFFFF)),
            children,
            dirty: Dirty { is_dirty: true },
            last_build: None,
            style: accepted_style
        }
    
    }

    pub fn display(&mut self, sc: &mut Screen) {
        let (buf, _,_ ) = self.draw(sc.size());
        sc.update(buf);
    }

}

impl Widget for Body {

    fn force_build(&mut self, par_size: (usize, usize)) {
            self.buf.resize_if_needed(par_size.0, par_size.1);
            self.flush();

            for child in self.children.iter() {
                child.borrow_mut().update(par_size);
            }

            let mut wrapper = Wrapper {
                layout: PositionLayout {},
                children: self.children.clone()
            };

            wrapper.render(&mut self.buf);

            self.dirty.clear();
            self.last_build = Some(par_size);

    }

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
        (self.buf.read(), w,h)

    }

    fn flush(&mut self) {
        self.buf.flush();
        self.dirty.mark();
    }

    fn update_dirty_state(&mut self, par_size: (usize, usize)) {
        
        if self.last_build != Some(par_size) {
            self.dirty.mark();
        }else {
            self.dirty.clear();
        }

        for child in self.children.iter() {
            child.borrow_mut().update_dirty_state(par_size);
        }

    }

    fn is_dirty(&self) -> bool {

        if self.dirty.check() {
            return true;
        }
        for child in self.children.iter(){
            if child.borrow().is_dirty() {
                return true;
            }
        }

        false  
    }

    fn style(&self) -> Rc<Style> {
        self.style.clone()
    }
}

impl DefStyle for Body {

    fn default_style() -> Style {
        Style::new().color(Color::rgba_to_u32(255, 255, 255, 255))
    }
}