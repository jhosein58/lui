extern crate lui;


use lui::*;

fn main () {

    let mut sc = Screen::new("hello", 300, 300);

    let style = Style::new()
    .color(0xFFFF0000)
    .width(Size::Absolute(100))
    .height(Size::Absolute(100))
    .get();

    let mut buf = Body::new(vec![

        Rectangle::new(style.clone()),
        Rectangle::new(style.clone()),
        Rectangle::new(style.clone()),


    ], None);

    while sc.is_open() {
        buf.display(&mut sc);
    }

}
