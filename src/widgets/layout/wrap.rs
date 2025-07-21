use std::{cell::RefCell, rc::Rc};

use crate::{Layout, PositionEntry, PositionType, Widget};

pub struct WrapLayout {
    pub spacing: usize,
}

impl Layout for WrapLayout {
    fn compute_positions(&self, children: Rc<Vec<Rc<RefCell<dyn Widget>>>>, area: (usize, usize)) -> Vec<PositionEntry> {
        let mut positions = Vec::new();
        let (max_width, _) = area;

        let mut x = 0;
        let mut y = 0;
        let mut line_height = 0;

        for child in children.iter() {
            let size = child.borrow().size();
            let (w, h) = size;

            if x + w > max_width {
                x = 0;
                y += line_height + self.spacing;
                line_height = 0;
            }

            positions.push(PositionEntry {
                widget: child.clone(),
                position: Some((x, y)),
                layout_type: PositionType::Fallback,
            });

            x += w + self.spacing;
            line_height = line_height.max(h);
        }

        positions
    }
}
