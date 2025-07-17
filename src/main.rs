extern crate lui;

use lui::*;

fn main () {

    let mut sc = Screen::new("hello", 300, 300);
    
    let mut buf = Body::new(300, 300, vec![
        Rectangle::new(Style::new().set_color(Color::rgb_to_u32(0, 255, 0)).set_width(Relative(0.5)).set_height(Absolute(200)))
    ]);

    let mut fps = Fps::start();

    while sc.is_open() {
        let (_,_, b) = buf.draw(sc.size());
        sc.update(b);
        buf.flush();

        fps.tick();
        println!("{:?}", fps.read());
    }
}