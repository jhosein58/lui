use std::rc::Rc;

use lui::{graphics::screen::Screen, *};

fn main() {
    

    let buf = Body::new(Rc::new(vec![

        Container::new(Rc::new(vec![

            Container::new(Rc::new(vec![

                Container::new(Rc::new(vec![

                    Text::new("Services", Style::new()
                    .font_size(20.0)
                    .position(Position::Absolute(PosVal::Absolute(10), PosVal::Absolute(10)))
                    .get())

                ]), Style::new()
                .width(Size::Relative(1.0))
                .height(Size::Absolute(40))
                .get()),

                Container::new(Rc::new(vec![


                    Container::new(Rc::new(vec![
                        Container::new(Rc::new(vec![

                            Image::new("assets/1.png", Style::new()
                            .width(Size::Absolute(50))
                            .height(Size::Absolute(50))
                            .position(Position::center())
                            .get())

                        ]), Style::new()
                        .position(Position::Absolute(
                            PosVal::Expr(Box::new(|sw,_,pw,_| ((pw - sw) / 2) )), 
                            PosVal::Expr(Box::new(|_: usize,sh,_,ph| ((ph - sh) / 2) - 20))
                        ))
                        .width(Size::Absolute(60))
                        .height(Size::Absolute(60))
                        .color(Color::rgba_to_u32(124, 47, 148, 255))
                        .border_radius(16)
                        .get()),
                        Text::new("Title", Style::new()
                        .position(Position::Absolute(
                            PosVal::Expr(Box::new(|sw,_,pw,_| ((pw - sw) / 2) )), 
                            PosVal::Expr(Box::new(|_: usize,sh,_,ph| ((ph - sh) / 2) + 30))
                        ))
                        .font_size(18.0)
                        .get()),
                    ]), Style::new()
                    .width(Size::Relative(0.2))
                    .height(Size::Relative(1.0))
                    .get()),

                    Container::new(Rc::new(vec![
                        Container::new(Rc::new(vec![

                            Image::new("assets/2.png", Style::new()
                            .width(Size::Absolute(50))
                            .height(Size::Absolute(50))
                            .position(Position::center())
                            .get())

                        ]), Style::new()
                        .position(Position::Absolute(
                            PosVal::Expr(Box::new(|sw,_,pw,_| ((pw - sw) / 2) )), 
                            PosVal::Expr(Box::new(|_: usize,sh,_,ph| ((ph - sh) / 2) - 20))
                        ))
                        .width(Size::Absolute(60))
                        .height(Size::Absolute(60))
                        .color(Color::rgba_to_u32(124, 47, 148, 255))
                        .border_radius(16)
                        .get()),
                        Text::new("Title", Style::new()
                        .position(Position::Absolute(
                            PosVal::Expr(Box::new(|sw,_,pw,_| ((pw - sw) / 2) )), 
                            PosVal::Expr(Box::new(|_: usize,sh,_,ph| ((ph - sh) / 2) + 30))
                        ))
                        .font_size(18.0)
                        .get()),
                    ]), Style::new()
                    .width(Size::Relative(0.2))
                    .height(Size::Relative(1.0))
                    .get()),

                    Container::new(Rc::new(vec![
                        Container::new(Rc::new(vec![

                            Image::new("assets/3.png", Style::new()
                            .width(Size::Absolute(50))
                            .height(Size::Absolute(50))
                            .position(Position::center())
                            .get())

                        ]), Style::new()
                        .position(Position::Absolute(
                            PosVal::Expr(Box::new(|sw,_,pw,_| ((pw - sw) / 2) )), 
                            PosVal::Expr(Box::new(|_: usize,sh,_,ph| ((ph - sh) / 2) - 20))
                        ))
                        .width(Size::Absolute(60))
                        .height(Size::Absolute(60))
                        .color(Color::rgba_to_u32(124, 47, 148, 255))
                        .border_radius(16)
                        .get()),
                        Text::new("Title", Style::new()
                        .position(Position::Absolute(
                            PosVal::Expr(Box::new(|sw,_,pw,_| ((pw - sw) / 2) )), 
                            PosVal::Expr(Box::new(|_: usize,sh,_,ph| ((ph - sh) / 2) + 30))
                        ))
                        .font_size(18.0)
                        .get()),
                    ]), Style::new()
                    .width(Size::Relative(0.2))
                    .height(Size::Relative(1.0))
                    .get()),

                    Container::new(Rc::new(vec![
                        Container::new(Rc::new(vec![

                            Image::new("assets/4.png", Style::new()
                            .width(Size::Absolute(50))
                            .height(Size::Absolute(50))
                            .position(Position::center())
                            .get())

                        ]), Style::new()
                        .position(Position::Absolute(
                            PosVal::Expr(Box::new(|sw,_,pw,_| ((pw - sw) / 2) )), 
                            PosVal::Expr(Box::new(|_: usize,sh,_,ph| ((ph - sh) / 2) - 20))
                        ))
                        .width(Size::Absolute(60))
                        .height(Size::Absolute(60))
                        .color(Color::rgba_to_u32(124, 47, 148, 255))
                        .border_radius(16)
                        .get()),
                        Text::new("Title", Style::new()
                        .position(Position::Absolute(
                            PosVal::Expr(Box::new(|sw,_,pw,_| ((pw - sw) / 2) )), 
                            PosVal::Expr(Box::new(|_: usize,sh,_,ph| ((ph - sh) / 2) + 30))
                        ))
                        .font_size(18.0)
                        .get()),
                    ]), Style::new()
                    .width(Size::Relative(0.2))
                    .height(Size::Relative(1.0))
                    .get()),

                    Container::new(Rc::new(vec![
                        Container::new(Rc::new(vec![

                            Image::new("assets/5.png", Style::new()
                            .width(Size::Absolute(50))
                            .height(Size::Absolute(50))
                            .position(Position::center())
                            .get())

                        ]), Style::new()
                        .position(Position::Absolute(
                            PosVal::Expr(Box::new(|sw,_,pw,_| ((pw - sw) / 2) )), 
                            PosVal::Expr(Box::new(|_: usize,sh,_,ph| ((ph - sh) / 2) - 20))
                        ))
                        .width(Size::Absolute(60))
                        .height(Size::Absolute(60))
                        .color(Color::rgba_to_u32(124, 47, 148, 255))
                        .border_radius(16)
                        .get()),
                        Text::new("Title", Style::new()
                        .position(Position::Absolute(
                            PosVal::Expr(Box::new(|sw,_,pw,_| ((pw - sw) / 2) )), 
                            PosVal::Expr(Box::new(|_: usize,sh,_,ph| ((ph - sh) / 2) + 30))
                        ))
                        .font_size(18.0)
                        .get()),
                    ]), Style::new()
                    .width(Size::Relative(0.2))
                    .height(Size::Relative(1.0))
                    .get()),
                    
                ]), Style::new()
                .width(Size::Relative(1.0))
                .height(Size::Absolute(125))
                .get()),
            ]), Style::new()
        .width(Size::Relative(0.95))
        .height(Size::Absolute(175))
        .position(Position::center())
        .border_radius(12)
        .color(Color::rgba_to_u32(225, 225, 225, 255))
        .get()),
        ]), Style::new()
        .width(Size::Relative(1.0))
        .height(Size::Absolute(200))
        .get()),

        Container::new(Rc::new(vec![
            Text::new("LIVES", Style::new()
            .position(Position::center())
            .get())
        ]), Style::new()
        .width(Size::Absolute(100))
        .height(Size::Absolute(50))
        .get()),

        Container::new(Rc::new(vec![

            Container::new(Rc::new(vec![
                Container::new(Rc::new(vec![
                    Container::new(Rc::new(vec![
                    ]), Style::new().width(Size::Absolute(80)).height(Size::Absolute(80)).border_radius(40).position(Position::center()).color(0xFFFFFFFF).get())
                ]), Style::new()
            .width(Size::Absolute(90))
            .height(Size::Absolute(90))
            .color(0xFFde1d8d)
            .position(Position::center())
            .border_radius(45)
            .get()),

            ]), Style::new()
            .width(Size::Absolute(100))
            .height(Size::Relative(1.0))
            .get()),

            Container::new(Rc::new(vec![
                Text::new("Test Title", Style::new().position(Position::center()).get())
            ]), Style::new().width(Size::Absolute(100)).height(Size::Relative(1.0)).get())

        ]), Style::new()
        .width(Size::Relative(1.0))
        .height(Size::Absolute(100))
        .get()),

                Container::new(Rc::new(vec![

            Container::new(Rc::new(vec![
                Container::new(Rc::new(vec![
                    Container::new(Rc::new(vec![
                    ]), Style::new().width(Size::Absolute(80)).height(Size::Absolute(80)).border_radius(40).position(Position::center()).color(0xFFFFFFFF).get())
                ]), Style::new()
            .width(Size::Absolute(90))
            .height(Size::Absolute(90))
            .color(0xFFde1d8d)
            .position(Position::center())
            .border_radius(45)
            .get()),

            ]), Style::new()
            .width(Size::Absolute(100))
            .height(Size::Relative(1.0))
            .get()),

            Container::new(Rc::new(vec![
                Text::new("Test Title", Style::new().position(Position::center()).get())
            ]), Style::new().width(Size::Absolute(100)).height(Size::Relative(1.0)).get())

        ]), Style::new()
        .width(Size::Relative(1.0))
        .height(Size::Absolute(100))
        .get()),

                Container::new(Rc::new(vec![

            Container::new(Rc::new(vec![
                Container::new(Rc::new(vec![
                    Container::new(Rc::new(vec![
                    ]), Style::new().width(Size::Absolute(80)).height(Size::Absolute(80)).border_radius(40).position(Position::center()).color(0xFFFFFFFF).get())
                ]), Style::new()
            .width(Size::Absolute(90))
            .height(Size::Absolute(90))
            .color(0xFFde1d8d)
            .position(Position::center())
            .border_radius(45)
            .get()),

            ]), Style::new()
            .width(Size::Absolute(100))
            .height(Size::Relative(1.0))
            .get()),

            Container::new(Rc::new(vec![
                Text::new("Test Title", Style::new().position(Position::center()).get())
            ]), Style::new().width(Size::Absolute(100)).height(Size::Relative(1.0)).get())

        ]), Style::new()
        .width(Size::Relative(1.0))
        .height(Size::Absolute(100))
        .get()),
        

    ]), None);

    Screen::default().display(buf);
}
