extern crate lui;
use lui::*;

fn main () {

    let mut sc = Screen::new("hello", 300, 300);
    let mut buf = GBuf::new(300, 300, Color::rgb_to_u32(255, 128, 0));

    while sc.is_open() {
        let (w, h) = sc.size();
        buf.resize_if_needed(w, h);
        sc.update(buf.read());
    }
}