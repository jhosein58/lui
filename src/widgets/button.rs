use std::rc::Rc;

use crate::{widgets::{helpers::default_style::DefaultStyle, props::dirty::Dirty}, Container, DefStyle, Font, Pos, PosKind, Size, Style, Text, Widget};

pub struct Button {
    child: Box<Container>,
    dirty: Dirty,
    style: Rc<Style>, 
}

impl Button {

    pub fn new(s: &str, style: Option<Rc<Style>>) -> Box<Self> {

        let style = DefaultStyle::optional_style::<Self>(style);

        let br = style.get_border_radius().unwrap_or(12);
        let tc = style.get_color().unwrap_or(0xFFFFFFFF);
        let bg = style.get_background().unwrap_or(0xFF4681f4);
        let fs = style.get_font_size().unwrap_or(16.0);


        let t_text = Text::new(s, Style::new().font_size(fs).color(tc).get());
        let (tw, th) = t_text.size();

        let w_val = tw + 30;
        let h_val = th + 10;

        let t_x = (w_val - tw) / 2;
        let t_y = (h_val - th) / 2;
        
        let text = Text::new(s, Style::new().font_size(fs).color(tc).position(Pos::Pos(PosKind::Relative(t_x), PosKind::Relative(t_y))).get());
        let w = style.get_width().unwrap_or(Size::Absolute(w_val));
        let h = style.get_height().unwrap_or(Size::Absolute(h_val));

        
        let child  = Container::new(vec![
            text
        ], Style::new().color(bg).width(w).height(h).border_radius(br).get());

        Box::new(Self { 
            child,
            dirty: Dirty { is_dirty: true },
            style,
        })
    }
}

impl Widget for Button {
    fn size(&self) -> (usize, usize) {
        self.child.size()
    }
    fn flush(&mut self) {
        self.child.flush();
        self.dirty.mark();
    }
    fn is_dirty(&self) -> bool {
        self.child.is_dirty()
    }
    fn style(&self) -> Rc<Style> {
        self.style.clone()
    }
    fn update_dirty_state(&mut self, par_size: (usize, usize)) {
        self.child.update_dirty_state(par_size);
        self.dirty.is_dirty = self.child.is_dirty()
    }
    fn update(&mut self, par_size: (usize, usize)) {
        self.child.update(par_size);
    }
    fn draw(&mut self, par_size: (usize, usize)) -> (&Vec<u32>, usize, usize) {
        self.child.draw(par_size)
    }
}

impl DefStyle for Button {
    fn default_style() -> Style {
        Style::new()

        .border        ((2, 0xFFFF0000))
        .border_radius (8)
        .color         (0xFFFFFFFF)
        .background    (0xFF4681f4)
        .font          (Font::default())
        .font_size     (28.0)
    }
}