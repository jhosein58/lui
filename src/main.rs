extern crate lui;

use lui::*;

fn main () {
    
    let mut sc = Screen::new("test", 200, 200);

    while sc.is_open() {
        let (w, h) = sc.size();
        let buf = vec![0u32; w * h];
        sc.update(&buf);
    }
}