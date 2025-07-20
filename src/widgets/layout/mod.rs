
use crate::{Widget};

pub mod wrapper;
pub mod column_layout;
pub mod row_layout;

pub trait Layout {
    fn compute_positions(&self, children: &[Box<dyn Widget>], area: (usize, usize)) -> Vec<(usize, usize)>;
}
