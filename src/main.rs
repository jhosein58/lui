extern crate lui;


use lui::*;

fn main () {

    let mut sc = Screen::new("hello", 300, 300);

    let mut buf = Body::new(vec![

        Container::new(vec![], Style::new().width(Size::Absolute(100)).height(Size::Absolute(100)).color(0xFFFF0000).get()),


    ], None);

    while sc.is_open() {
        buf.display(&mut sc);
    }

}
