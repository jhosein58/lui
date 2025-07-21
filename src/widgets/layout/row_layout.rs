
use std::{cell::RefCell, rc::Rc};

use crate::{Layout, PositionEntry, PositionType, Widget};

pub struct RowLayout {
    pub spacing: usize,
}

impl Layout for RowLayout {
    fn compute_positions(&self, children: Rc<Vec<Rc<RefCell<dyn Widget>>>>, _: (usize, usize)) -> Vec<PositionEntry> {

        let mut positions = Vec::new();
        let mut x = 0;

        for child in children.iter() {

            let (w, _) = child.borrow().size();

            positions.push(PositionEntry {

                widget: child.clone(),
                position: Some((x,0)),
                layout_type: PositionType::Direct
                
            });

            x += w + self.spacing;
        }

        positions
    }
}