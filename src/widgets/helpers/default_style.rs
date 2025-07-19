use std::{rc::Rc};

use crate::{DefStyle, Style};

pub struct DefaultStyle { }


impl DefaultStyle {
    
    pub fn optional_style<T>(style: Option<Rc<Style>>) -> Rc<Style> 
    where T: DefStyle
    {
        
        if let Some(style_val) = style {
            return style_val;
        }

        T::default_style().get().unwrap()

    }
}