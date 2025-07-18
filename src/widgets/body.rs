
use crate::{widgets::{props::dirty::Dirty, Widget}, Color, GBuf, Screen, Style};


pub struct Body {
    buf: GBuf,
    children: Vec<Box<dyn Widget>>,
    dirty: Dirty,
    last_build: Option<(usize, usize)>
} 

impl Body {

    pub fn new(children: Vec<Box<dyn Widget>>) -> Self {
        Self { 
            buf: GBuf::new(0, 0, Color::rgba_to_u32(255, 255, 255, 255)),
            children,
            dirty: Dirty { is_dirty: true },
            last_build: None
        }
    
    }

    pub fn display(&mut self, sc: &mut Screen) {
        let (_,_, buf) = self.draw(sc.size());
        sc.update(buf);
    }
}

impl Widget for Body {

    fn size(&self) -> (usize, usize) {
        self.buf.size()
    }

    fn update(&mut self, par_size: (usize, usize)) {
        
        self.update_dirty_state(par_size);

        if self.is_dirty() {

            
            self.buf.resize_if_needed(par_size.0, par_size.1);
            self.flush();

            let mut biggest_h = 0;
            let mut current_x = 0;
            let mut current_y = 0;

            for child in self.children.iter_mut() {
                let buf = child.draw(par_size);


                if current_x + buf.0 > par_size.0 {
                    current_x = 0;
                    current_y += biggest_h;
                    biggest_h = 0;          
                }


                self.buf.merge(current_x, current_y, buf).unwrap();


                current_x += buf.0;

                if buf.1 > biggest_h {
                    biggest_h = buf.1;
                }
            }

            self.dirty.clear();
            self.last_build = Some(par_size);
        }

    }

    fn draw(&mut self, par_size: (usize, usize)) -> (usize, usize, &Vec<u32>) {

        self.update(par_size);

        let (w, h) = self.buf.size();
        (w,h ,self.buf.read())

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

        for child in self.children.iter_mut() {
            child.update_dirty_state(par_size);
        }

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

    fn style(&self) -> crate::Style {
        Style::new()
    }
}