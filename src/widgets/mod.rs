use crate::Style;


pub mod props;

pub mod body;
pub mod nil;
pub mod rectangle;
pub mod container;

pub trait Widget {
    fn size(&self) -> (usize, usize);
    fn update(&mut self, par_size: (usize, usize));
    fn draw(&mut self, par_size: (usize, usize)) -> (usize, usize, &Vec<u32>);
    fn flush(&mut self);
    fn is_dirty(&self) -> bool;
    fn update_dirty_state(&mut self, par_size: (usize, usize));
    fn style(&self) -> Style;
}