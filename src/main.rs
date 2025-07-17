extern crate lui;
use std::{thread::sleep, time::Duration};

use lui::*;

fn main () {

    // let mut sc = Screen::new("hello", 300, 300);
    
    // let mut buf = Body::new(300, 300, vec![
    //     Rectangle::new(Style::new().set_color(Color::rgb_to_u32(0, 255, 0)).set_width(Relative(0.5)).set_height(Absolute(200)))
    // ]);


    // while sc.is_open() {
    //     sc.update(buf.draw(sc.size()));
    //     buf.flush();
    // }

    let mut fps = Fps::start();

    loop {
        fps.tick();
        println!("{:?}", fps.read());
    }
}