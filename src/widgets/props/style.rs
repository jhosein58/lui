
use crate::{DirVal, Font, Pos, Size};

#[derive(Clone, Copy)]
pub struct Style<'a> {
    color: Option<u32>,
    width: Option<Size>,
    height: Option<Size>,
    position: Option<Pos>,
    border: Option<(u32, usize)>,
    margin: Option<DirVal<usize>>,
    font_size: Option<Size>,
    font: Option<&'a Font>,
}

impl<'a> Style<'a> {

    pub fn new() -> Self {
        Self { 
            color:     None, 
            width:     None, 
            height:    None,
            position:  None,
            border:    None,
            margin:    None,
            font_size: None,
            font:      None
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

    pub fn set_position(mut self, p: Pos) -> Self {
        self.position = Some(p);
        self
    }
    pub fn get_position(&self) -> Option<Pos> {
        self.position
    }

    pub fn set_border(mut self, b: (u32, usize)) -> Self {
        self.border = Some(b);
        self
    }
    pub fn get_border(&self) -> Option<(u32, usize)> {
        self.border
    }

    pub fn set_margin(mut self, m: DirVal<usize>) -> Self {
        self.margin = Some(m);
        self
    }
    pub fn get_margin(&self) -> Option<DirVal<usize>> {
        self.margin
    }

    pub fn set_font_size(mut self, f: Size) -> Self {
        self.font_size = Some(f);
        self
    }
    pub fn get_font_size(&self) -> Option<Size> {
        self.font_size
    }
}