
pub mod border_radius;

pub trait Processor {
    fn process(&self, buf: &mut Vec<u32>, w: usize, h: usize);
}