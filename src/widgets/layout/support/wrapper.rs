
use std::{cell::RefCell, rc::Rc};

use crate::{widgets::{layout::Layout}, GBuf, Widget};


pub struct Wrapper<L: Layout> {
    pub layout: L,
    pub children: Rc<Vec<Rc<RefCell<dyn Widget>>>>,
}

impl<L: Layout> Wrapper<L> {
    pub fn render(&mut self, parent_buffer: &mut GBuf) {
        let parent_size = parent_buffer.size();
        let mut positions = self.layout.compute_positions(self.children.clone(), parent_size);

        for pe in positions.iter_mut() {
            if let Some((x, y)) = &pe.position {
                let mut w = pe.widget.borrow_mut();
                let child_buf = w.draw(parent_size);
                parent_buffer.merge(*x, *y, child_buf);
            }
        }
    }
}
