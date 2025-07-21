extern crate lui;


use std::rc::Rc;

use lui::*;

fn main () {

    let mut sc = Screen::new("hello", 300, 300);


    let mut buf = Body::new( Rc::new(vec![
        Button::new("click!", Style::new().position(Position::center()).get()),
        
    ]), None);


    while sc.is_open() {

        buf.display(&mut sc);
    }

}
