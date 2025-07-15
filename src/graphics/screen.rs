use minifb::{Window, WindowOptions};


pub struct Screen {
    window: Window,
}

impl Screen {

    pub fn new(name: &str, w: usize, h: usize) -> Self{
        Self { 
            window: Window::new(name, w, h, WindowOptions {
                resize: true,
                ..WindowOptions::default()
            }).expect("Err: Cannot create new screen.")
        }
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }
    pub fn size(&self) -> (usize, usize) {
        self.window.get_size()
    }

    pub fn update(&mut self, buf: &[u32]) {
        let (w, h) = self.size();
        self.window.update_with_buffer(buf, w, h)
        .expect("Err: Updating buffer failed");
    }
}