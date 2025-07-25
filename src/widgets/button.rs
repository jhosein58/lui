// use std::{cell::RefCell, rc::Rc};

// use crate::{widgets::{helpers::default_style::DefaultStyle, props::dirty::Dirty}, Container, DefStyle, Font, Position, Size, Style, Text, Widget};

// pub struct Button {
//     child: Container,
//     dirty: Dirty,
//     style: Rc<Style>, 
// }

// impl Button {

//     pub fn new(s: &str, style: Option<Rc<Style>>) -> Rc<RefCell<Self>> {

//         let style = DefaultStyle::optional_style::<Self>(style);

//         let br = style.get_border_radius().unwrap_or(8);
//         let tc = style.get_color().unwrap_or(0xFFFFFFFF);
//         let bg = style.get_background().unwrap_or(0xFF4681f4);
//         let fs = style.get_font_size().unwrap_or(28.0);

//         let text = Text::new(s, Style::new().color(tc).font_size(fs).position(Position::center()).get());
//         let (tw, th) = text.borrow().size();
//         let child  = Container::new_self(Rc::new(vec![
//             text
//         ]), Style::new().color(bg).width(Size::Absolute(tw + 35)).height(Size::Absolute(th + 15)).border_radius(br).get());

//         Rc::new(RefCell::new(Self { 
//             child,
//             dirty: Dirty { is_dirty: true },
//             style,
//         }))
//     }
// }

// impl Widget for Button {
//     fn force_build(&mut self, par_size: (usize, usize)) {
//         self.child.force_build(par_size);
//     }
//     fn size(&self) -> (usize, usize) {
//         self.child.size()
//     }
//     fn flush(&mut self) {
//         self.child.flush();
//         self.dirty.mark();
//     }
//     fn is_dirty(&self) -> bool {
//         self.child.is_dirty()
//     }
//     fn style(&self) -> Rc<Style> {
//         self.style.clone()
//     }
//     fn update_dirty_state(&mut self, par_size: (usize, usize)) {
//         self.child.update_dirty_state(par_size);
//         self.dirty.is_dirty = self.child.is_dirty()
//     }
//     fn update(&mut self, par_size: (usize, usize)) {
//         self.child.update(par_size);
//     }
//     fn draw(mut self, par_size: (usize, usize)) -> (&Vec<u32>, usize, usize) {
//         self.child.draw(par_size);
//     }
// }

// impl DefStyle for Button {
//     fn default_style() -> Style {
//         Style::new()

//         .border        ((2, 0xFFFF0000))
//         .border_radius (8)
//         .color         (0xFFFFFFFF)
//         .background    (0xFF4681f4)
//         .font          (Font::default())
//         .font_size     (28.0)
//     }
// }