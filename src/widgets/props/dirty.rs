pub struct Dirty {
    pub is_dirty: bool,
}

impl Dirty {
    pub fn mark(&mut self) {
        self.is_dirty = true;
    }

    pub fn clear(&mut self) {
        self.is_dirty = false;
    }

    pub fn check(&self) -> bool {
        self.is_dirty
    }
}
