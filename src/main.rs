extern crate lui;


use lui::*;

fn main () {

    let style = Style::new()
    
    .font_size(32.0)
    .color(0xFF0000)

    .get();


    let mut sc = Screen::new("hello", 300, 300);

    let mut buf = Body::new(vec![

        Text::new("AHMADI", style.clone()),
        Rectangle::new(Style::new().width(Size::Absolute(150)).height(Size::Absolute(150)).color(0xFFFF00FF).get()),
        Text::new("jhosein58", style)

    ], None);

    while sc.is_open() {
        buf.display(&mut sc);
    }

}
