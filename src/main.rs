extern crate lui;


use lui::*;

fn main () {

    let style_1 = Style::new()
    
    .font_size(32.0)
    .color(0xFF0000)

    .get();


    let style_2 = Style::new()
    
    .width(Size::Absolute(150))
    .height(Size::Absolute(150))
    .color(0xFFFF00FF)
    
    .get();

    let style_3 = Style::new()
    
    .width(Size::Relative(1.0))
    .height(Size::Absolute(100))
    .color(Color::rgba_to_u32(0, 120, 120, 120))
    
    .get();


    let mut sc = Screen::new("hello", 300, 300);

    let mut buf = Body::new(vec![


        Text::new("AHMADI", style_1.clone()),

        Rectangle::new(style_2),

        Text::new("jhosein58", style_1.clone()),

        Container::new(vec![], Style::new().width(Size::Relative(1.0)).height(Size::Absolute(20)).get()),

        Container::new(vec![
            
            Text::new("Hello", Style::new().font_size(28.0).position(Pos::Pos(PosKind::Percentage(0.4), PosKind::Relative(30))).get()),

        ], style_3)


    ], None);

    while sc.is_open() {
        buf.display(&mut sc);
    }

}
