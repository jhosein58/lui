use minifb::{Error, Window, WindowOptions};


pub struct Screen {
    window: Window,
}

impl Screen {

    pub fn new(name: &str, w: usize, h: usize) -> Result<Self, Error> {
        Ok(Self { 
            window: Window::new(name, w, h, WindowOptions {
                resize: true,
                ..WindowOptions::default()
            })?
        })
    }

    pub fn is_open(&self) -> bool {
        self.window.is_open()
    }

    pub fn size(&self) -> (usize, usize) {
        self.window.get_size()
    }

    pub fn update(&mut self, buf: &[u32]) -> Result<(), Error> {
        let (w, h) = self.size();
        self.window.update_with_buffer(buf, w, h)?;
        Ok(())
        
    }
}