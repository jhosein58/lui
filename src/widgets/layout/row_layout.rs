
use crate::{Layout, Widget};

pub struct RowLayout {
    pub spacing: usize,
}

impl Layout for RowLayout {
    fn compute_positions(&self, children: &[Box<dyn Widget>], _: (usize, usize)) -> Vec<(usize, usize)> {

        let mut positions = Vec::new();
        let mut x = 0;

        for child in children {
            let (w, _) = child.size();
            positions.push((x, 0));
            x += w + self.spacing;
        }

        positions
    }
}