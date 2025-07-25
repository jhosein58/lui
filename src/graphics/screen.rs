use std::{cell::RefCell, rc::Rc, sync::Arc};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes},
};

use crate::{Body, Widget};

pub struct Screen {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    renderer: Option<Rc<RefCell<Body>>>,
}

impl Default for Screen {
    fn default() -> Self {
        env_logger::init();
        Self {
            window: None,
            pixels: None,
            renderer: None,
        }
    }
}

impl ApplicationHandler for Screen {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default().with_title("Screen"))
                .unwrap(),
        );

        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, window.clone());
        let pixels = Pixels::new(size.width, size.height, surface).unwrap();

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(pixels) = &mut self.pixels {
                    let _ = pixels.resize_surface(size.width, size.height);
                    let _ = pixels.resize_buffer(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(pixels), Some(window), Some(renderer)) =
                    (&mut self.pixels, &self.window, &self.renderer)
                {
                    let (w, h) = {
                        let size = window.inner_size();
                        (size.width as usize, size.height as usize)
                    };

                    let mut body = renderer.borrow_mut();
                    body.tick();

                    body.update_dirty_state((w, h));
                    let is_dirty = body.is_dirty();

                    if is_dirty {

                        body.force_build((w, h));

                        let buffer = body.draw((w, h));
                        let frame = pixels.frame_mut();

                        for (i, px) in frame.chunks_exact_mut(4).enumerate() {
                            let color = buffer.0.get(i).copied().unwrap_or(0xFF000000);
                            px.copy_from_slice(&[
                                ((color >> 16) & 0xFF) as u8, // R
                                ((color >> 8) & 0xFF) as u8,  // G
                                (color & 0xFF) as u8,         // B
                                ((color >> 24) & 0xFF) as u8, // A
                            ]);
                        }
                        pixels.render().unwrap();
                    }

                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            window.request_redraw(); 
        }

    }
}

impl Screen {
    pub fn display(mut self, renderer: Rc<RefCell<Body>>)
    {
        self.renderer = Some(renderer);
        let event_loop = EventLoop::new().unwrap();
        event_loop.run_app(&mut self).unwrap();
    }
}
