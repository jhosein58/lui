use crate::Size;

pub struct Style {
    color: Option<u32>,
    width: Option<Size>,
    height: Option<Size>,
}

impl Style {

    pub fn new() -> Self {
        Self { 
            color: None, 
            width: None, 
            height: None
        }
    }
    
    pub fn set_color(mut self, c: u32) -> Self {
        self.color = Some(c);
        self
    }
    pub fn get_color(&self) -> Option<u32> {
        self.color
    }

    pub fn set_width(mut self, w: Size) -> Self {
        self.width = Some(w);
        self
    }
    pub fn get_width(&self) -> Option<Size> {
        self.width
    }

    pub fn set_height(mut self, h: Size) -> Self {
        self.height = Some(h);
        self
    }
    pub fn get_height(&self) -> Option<Size> {
        self.height
    }
}