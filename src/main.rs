use std::rc::Rc;

use lui::{graphics::screen::Screen, *};

fn main() {
    

    let style = Style::new()
     .color(0xFF00FF00)
    .width(Size::Absolute(50))
    .height(Size::Absolute(50))
    .get();

    let rec = Rectangle::new(style.clone());
    let buf = Body::new(Rc::new(vec![
        rec.clone(),       
    ]), None);

    Screen::default().display(buf);
}
