use std::{cell::RefCell, rc::Rc};

use crate::{Layout, Position, PositionEntry, PositionLayout, Widget};


pub struct FallbackLayout<T> where T: Layout {
    pub default_layout: T,
    pub position_layout: PositionLayout
}

impl<T> Layout for FallbackLayout<T> where  T: Layout{

    fn compute_positions(&self, children: Rc<Vec<Rc<RefCell<dyn Widget>>>>, area: (usize, usize)) -> Vec<PositionEntry> {

        let (mut on_default, mut on_position) = (Vec::new(), Vec::new());
        
        for child in children.iter() {
            if let Position::Default = child.borrow().style().get_position().unwrap_or(&Position::Default) {
                on_default.push(child.clone());
                continue;
            }
            on_position.push(child.clone());
        }

        let mut computed_default = self.default_layout.compute_positions(Rc::new(on_default), area);
        let mut computed_position = self.position_layout.compute_positions(Rc::new(on_position), area);
        computed_default.append(&mut computed_position);
        computed_default
    }
}