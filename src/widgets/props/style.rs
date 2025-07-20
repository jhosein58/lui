
use std::{rc::Rc};

use crate::{DirVal, Font, Position, Size};

pub struct Style {
    pub color:         Option<u32>,
    pub background:    Option<u32>,
    pub width:         Option<Size>,
    pub height:        Option<Size>,
    pub position:      Option<Position>,
    pub border:        Option<(u32, usize)>,
    pub border_radius: Option<usize>,
    pub margin:        Option<DirVal<usize>>,
    pub font_size:     Option<f32>,
    pub font:          Option<Font>,
}

impl Style {

    pub fn new() -> Self {
        Self { 
                color:         None, 
                background:    None,
                width:         None, 
                height:        None,
                position:      None,
                border:        None,
                border_radius: None,
                margin:        None,
                font_size:     None,
                font:          None
            }
    }

    pub fn empty() -> Option<Rc<Style>> {
        Self::new().get()
    }
    
    pub fn get(self) -> Option<Rc<Self>> {
        Some(Rc::new(self))
    }

    pub fn color(mut self, c: u32) -> Self {
        self.color = Some(c);
        self
    }
    pub fn get_color(&self) -> Option<u32> {
        self.color
    }

    pub fn background(mut self, c: u32) -> Self {
        self.background = Some(c);
        self
    }
    pub fn get_background(&self) -> Option<u32> {
        self.background
    }

    pub fn most_have_color(&self, default: u32) -> u32 {
        if let Some(c) = self.color {
            return c;
        }
        default
    }

    pub fn width(mut self, w: Size) -> Self {
        self.width = Some(w);
        self
    }
    pub fn get_width(&self) -> Option<Size> {
        self.width
    }

    pub fn height(mut self, h: Size) -> Self {
        self.height = Some(h);
        self
    }
    pub fn get_height(&self) -> Option<Size> {
        self.height
    }

    pub fn position(mut self, p: Position) -> Self {
        self.position = Some(p);
        self
    }
    pub fn get_position(&self) -> Option<&Position> {
        if let Some(pos) = &self.position {
            return Some(pos);
        }
        None
    }

    pub fn border(mut self, b: (u32, usize)) -> Self {
        self.border = Some(b);
        self
    }
    pub fn get_border(&self) -> Option<(u32, usize)> {
        self.border
    }

    pub fn border_radius(mut self, b: usize) -> Self {
        self.border_radius = Some(b);
        self
    }
    pub fn get_border_radius(&self) -> Option<usize> {
        self.border_radius
    }

    pub fn margin(mut self, top: usize, bottom: usize, left: usize, right: usize) -> Self {
        self.margin = Some(DirVal {
            top,
            bottom,
            left,
            right
        });
        self
    }
    pub fn get_margin(&self) -> Option<DirVal<usize>> {
        self.margin
    }

    pub fn font_size(mut self, f: f32) -> Self {
        self.font_size = Some(f);
        self
    }
    pub fn get_font_size(&self) -> Option<f32> {
        self.font_size
    }

    pub fn font(mut self, f: Font) -> Self {
        self.font = Some(f);
        self
    }
    pub fn get_font(&self) -> Option<&Font> {
        if let Some(f) = &self.font {
            return Some(&f)
        }
        None
    }

    // pub fn generate_text_from_font(&self, txt: &str) -> Option<GBuf> {
    //     match &self.font {
    //         Some(f) => {
    //             let [r,g,b,_] = Color::u32_to_rgba(self.get_color().unwrap_or(Color::rgba_to_u32(0, 0, 0, 255)));
    //             Some(f.text(txt, self.get_font_size().unwrap_or(12.0), (r,g,b)))
    //         }
    //         _ => None
    //     }
    // }
}