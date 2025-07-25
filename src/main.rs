use std::rc::Rc;

use lui::{graphics::screen::Screen, *};

fn main() {
    

    let buf = Body::new(Rc::new(vec![
        Image::new("assets/music.png", Style::new().width(Size::Absolute(100)).height(Size::Absolute(100)).get())     
    ]), Style::new().color(0xFF000000).get());

    Screen::default().display(buf);
}
