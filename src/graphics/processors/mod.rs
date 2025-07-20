
pub mod border_radius;

pub trait Processor {
    fn process(&self, buf: Vec<u32>, w: usize, h: usize) -> (Vec<u32>, usize, usize);
}