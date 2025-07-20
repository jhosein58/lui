
use crate::{widgets::layout::Layout, Widget};

pub struct ColumnLayout {
    pub spacing: usize,
}

impl Layout for ColumnLayout {
    fn compute_positions(&self, children: &[Box<dyn Widget>], _: (usize, usize)) -> Vec<(usize, usize)> {

        let mut positions = Vec::new();
        let mut y = 0;

        for child in children {
            let (_, height) = child.size(); 
            positions.push((0, y)); 
            y += height + self.spacing;
        }

        positions
    }
}