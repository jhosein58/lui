extern crate lui;

use lui::*;
use minifb::Error;

fn main () -> Result<(), Error> {


    let mut sc = Screen::new("test", 200, 200)?;

    while sc.is_open() {
        let (w, h) = sc.size();
        let buf = vec![0u32; w * h];
        sc.update(&buf)?;
    }

    Ok(())
}