use std::{cell::RefCell, rc::Rc};
use image::{DynamicImage, GenericImageView, Rgba, RgbaImage};
use crate::{widgets::{helpers::default_style::DefaultStyle, props::dirty::Dirty}, Color, DefStyle, GBuf, Size, Style, Widget};


pub struct Image {
    buf: GBuf,
    dirty: Dirty,
    image: DynamicImage,
    w: Size,
    h: Size,
    last_build: Option<(usize, usize)>,
    initialized: bool,
    style: Rc<Style>
}

impl Image {

    pub fn new(path: &str, style: Option<Rc<Style>>) -> Rc<RefCell<Self>> {

        let style = DefaultStyle::optional_style::<Self>(style);
        let w = style.get_width().unwrap_or(Size::Relative(1.0));
        let h = style.get_height().unwrap_or(Size::Relative(1.0));


        let image = Self::open_or_blank(path);
        Rc::new(RefCell::new(
            Self {
                buf: GBuf::new(0, 0, 0),
                dirty: Dirty { is_dirty: true },
                image,
                w,
                h,
                last_build: None,
                initialized: false,
                style
            }
        ))
    }

    fn open_or_blank(path: &str) -> DynamicImage {
        match image::open(path) {
            Ok(img) => img,
            Err(_) => {
                let mut blank = RgbaImage::new(10, 10);
                for pixel in blank.pixels_mut() {
                    *pixel = Rgba([255, 255, 255, 255]);
                }
                DynamicImage::ImageRgba8(blank)
            }
        }
    }

    fn init_build_if_need(&mut self, par_size: (usize, usize)) {

        if !self.initialized {
            self.force_build(par_size);
            self.initialized = true;
        }

    }

}

impl Widget for Image {
    
    fn force_build(&mut self, par_size: (usize, usize)) {

        let par_w = match self.w {
            Size::Relative(w) => {
                (w * par_size.0 as f32) as usize
            },
            Size::Absolute(w) => w
        };

        let par_h = match self.h {
            Size::Relative(h) => (h * par_size.1 as f32) as usize,
            Size::Absolute(h) => h
        };

        self.buf.resize_if_needed(par_w, par_h);
        self.buf.flush();

        let dummy_img = self.image.clone();

        let (img_w, img_h) = dummy_img.dimensions();
        let aspect_ratio = img_h as f32 / img_w as f32;
        let new_w = par_w as u32;
        let new_h = (new_w as f32 * aspect_ratio) as u32;
        let dummy_img = dummy_img.resize(new_w, new_h, image::imageops::FilterType::Lanczos3);
        let img_buf = dummy_img.to_rgba8();

        let final_img_buf: Vec<u32> = img_buf.pixels().map(|p| {
            let [r, g, b, a] = p.0;
            Color::rgba_to_u32(r, g, b, a)
        }).collect();

        self.buf.merge(0, 0, (&final_img_buf, new_w as usize, new_h as usize));

        self.dirty.clear();
        self.last_build = Some((par_w, par_h));

    }

    fn size(&self) -> (usize, usize) {
        self.buf.size()
    }
    fn flush(&mut self) {
        self.buf.flush();
        self.dirty.mark();
    }
    fn is_dirty(&self) -> bool {
        self.dirty.check()
    }
    fn style(&self) -> Rc<Style> {
        self.style.clone()
    }
    fn tick(&mut self) {
        
    }
     fn update(&mut self, par_size: (usize, usize)) {
        
        self.update_dirty_state(par_size);

        if self.is_dirty() {
           self.force_build(par_size);
        }
        
    }
    fn update_dirty_state(&mut self, par_size: (usize, usize)) {
        self.init_build_if_need(par_size);

        let w_old = self.last_build.unwrap_or((0,0)).0;

        let w_need_update = if let Size::Relative(w_val) = self.w {
            w_old != ( w_val * par_size.0 as f32 ) as usize
        }else {
            false
        };
    

        let h_old = self.last_build.unwrap_or((0,0)).1;
        let h_need_update = if let Size::Relative(h_val) = self.h {
            h_old != ( h_val * par_size.1 as f32 ) as usize
        }else {
            false
        };


       if w_need_update || h_need_update{
        self.dirty.mark();
       }else {
           self.dirty.clear();
       }
    }

    fn draw(&mut self, par_size: (usize, usize)) -> (&Vec<u32>, usize, usize) {
        self.update(par_size);
        let (w, h) = self.buf.size();
        (self.buf.read(), w, h)
    }

}

impl DefStyle for Image {
    fn default_style() -> Style {
        Style::new().width(Size::Relative(1.0)).height(Size::Relative(1.0))
    }
}