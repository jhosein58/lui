use std::{cell::RefCell, rc::Rc};

use crate::{ Layout, PosVal, Position, PositionEntry, PositionType, Widget};



pub struct PositionLayout {

}

impl Layout for PositionLayout {
    fn compute_positions(&self, children: Rc<Vec<Rc<RefCell<dyn Widget>>>>, area: (usize, usize)) -> Vec<PositionEntry> {

        let mut positions = Vec::new();

        for child in children.iter() {

            let child_style = child.borrow().style();

            let widget_position = child_style.get_position().unwrap_or(&Position::Default);

            let (new_widget_position, widget_position_type);
            match widget_position {
                Position::Absolute(x, y) => {
                    let (w_x, w_y);
                    let w_size = child.borrow().size();

                    match x {
                        PosVal::Absolute(x_val) => {w_x = x_val.to_owned()},
                        PosVal::Expr(f) => {

                            w_x = f(w_size.0, w_size.1, area.0, area.1);
                        }
                    }

                    match y {
                        PosVal::Absolute(y_val) => {w_y = y_val.to_owned()},
                        PosVal::Expr(f) => {
                            w_y = f(w_size.0, w_size.1, area.0, area.1);
                        }
                    }

                    new_widget_position = Some((w_x, w_y));
                    widget_position_type = PositionType::Direct;
                    
                },
                Position::Default => {
                    new_widget_position = None;
                    widget_position_type = PositionType::Fallback;
                }
            }

            positions.push(PositionEntry { widget: child.clone(), position: new_widget_position, layout_type: widget_position_type });

        }

        positions
    }
}