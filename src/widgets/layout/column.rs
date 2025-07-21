
use std::{cell::RefCell, rc::Rc};

use crate::{Layout, PositionEntry, PositionType, Widget};

pub struct ColumnLayout {
    pub spacing: usize,
}

impl Layout for ColumnLayout {
    fn compute_positions(&self, children:Rc<Vec<Rc<RefCell<dyn Widget>>>>, _: (usize, usize)) -> Vec<PositionEntry> {

        let mut positions = Vec::new();
        let mut y = 0;

        for child in children.iter() {
            let (_, height) = child.borrow().size(); 
            positions.push(PositionEntry { 
                widget: child.clone(), 
                position: Some((0, y)), 
                layout_type: PositionType::Direct 
            });
            y += height + self.spacing;
        }

        positions
    }
}