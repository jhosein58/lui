extern crate lui;
use lui::*;

fn main () {

    let mut sc = Screen::new("hello", 300, 300);

    while sc.is_open() {
        let (w, h) = sc.size();
        let buf = vec![Color::rgb_to_u32(255, 0, 0); w*h];
        sc.update(&buf);
    }
}