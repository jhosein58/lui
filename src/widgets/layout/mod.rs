
use crate::{Widget};

pub mod column_layout;
pub mod wrapper;

pub trait Layout {
    fn compute_positions(&self, children: &[Box<dyn Widget>], area: (usize, usize)) -> Vec<(usize, usize)>;
}
