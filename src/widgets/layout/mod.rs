use std::rc::Rc;

use crate::{Pos, Widget};

pub trait Layout {
    fn compute_positions(&self, children: &[Rc<dyn Widget>], area: (usize, usize)) -> Vec<Pos>;
}
