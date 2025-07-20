extern crate lui;


use lui::{Button, *};

fn main () {

    let mut sc = Screen::new("hello", 300, 300);

    let mut buf = Body::new(vec![

        Container::new(vec![

            Text::new("Hello,world!", Style::new().font_size(32.0).position(Pos::Pos(PosKind::Relative(20), PosKind::Relative(20))).get()),
            Button::new("CLICK", Style::new().font_size(28.0).border_radius(4).position(Pos::Pos(PosKind::Relative(20), PosKind::Relative(60))).get()),

        ], Style::new().color(0xFF_f1d6ff).get())


    ], None);

    while sc.is_open() {
        buf.display(&mut sc);
    }

}
