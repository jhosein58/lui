use std::rc::Rc;

use crate::Style;


pub mod props;
pub mod helpers;

pub mod body;
pub mod nil;
pub mod rectangle;
pub mod container;
pub mod rawbuf;
pub mod text;

pub trait Widget {
    fn size(&self) -> (usize, usize);
    fn update(&mut self, par_size: (usize, usize));
    fn draw(&mut self, par_size: (usize, usize)) -> (usize, usize, &Vec<u32>);
    fn flush(&mut self);
    fn is_dirty(&self) -> bool;
    fn update_dirty_state(&mut self, par_size: (usize, usize));
    fn style(&self) -> Rc<Style>;
}

pub trait DefStyle {
    fn default_style() -> Style;
}