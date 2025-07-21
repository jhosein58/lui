
use std::{cell::RefCell, rc::Rc};

use crate::{Widget};

pub mod config;
pub mod wrapper;
pub mod column_layout;
pub mod row_layout;


pub struct PositionEntry {
    pub widget: Rc<RefCell<dyn Widget>>,  
    pub position: Option<(usize, usize)>, 
    pub layout_type: PositionType,
}

pub enum PositionType {
    Direct,       
    Fallback,    
    Hidden,      
}


pub trait Layout {
    fn compute_positions(&self, children: Rc<Vec<Rc<RefCell<dyn Widget>>>>, area: (usize, usize)) -> Vec<PositionEntry>;
}

