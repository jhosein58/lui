
use crate::{widgets::layout::Layout, GBuf, Widget};


pub struct Wrapper<L: Layout> {
    pub layout: L,
    pub children: Vec<Box<dyn Widget>>,
}

impl<L: Layout> Wrapper<L> {
    pub fn render(&mut self, parent_buffer: &mut GBuf) {
        let parent_size = parent_buffer.size();
        let positions = self.layout.compute_positions(&self.children, parent_size);

        for (widget, (x, y)) in self.children.iter_mut().zip(positions.iter()) {
            let child_buf = widget.draw(parent_size);
            parent_buffer.merge(*x, *y, child_buf);
        }
    }
}
