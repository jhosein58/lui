extern crate lui;

use lui::*;

fn main () {

    let mut sc = Screen::new("hello", 300, 300);
    
    
    let mut buf = Body::new( vec![
        Container::new(vec![

            Rectangle::new(Style::new()
            .set_color(Color::rgba_to_u32(0, 255, 255, 255))
            .set_width(Size::Absolute(50))
            .set_height(Size::Absolute(50))
            .set_position(Pos::Pos(PosKind::Percentage(0.5), PosKind::Percentage(0.5)))),
      
        ], Style::new()
        .set_color(Color::rgba_to_u32(255, 255, 0, 100))
        .set_height(Size::Absolute(200))
        .set_width(Size::Relative(0.5)))
    ]);


    while sc.is_open() {
        buf.display(&mut sc);
    }

}
