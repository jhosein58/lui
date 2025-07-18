extern crate lui;


use lui::{graphics::font::Font, *};

fn main () {

    let font = Font::new();

    let mut sc = Screen::new("hello", 300, 300);
    
    let buf = font.text("click!", 35.0, (255, 0, 0));
    
    let (w, h) = buf.size();
    let mut buf = Body::new( vec![
        Container::new(vec![

            RawBuf::new(buf.read().clone(), w, h)
      
        ], Style::new()
        .set_color(Color::rgba_to_u32(255, 255, 0, 100))
        .set_height(Size::Absolute(200))
        .set_width(Size::Relative(0.5)))
    ]);


    while sc.is_open() {
        buf.display(&mut sc);
    }

}
