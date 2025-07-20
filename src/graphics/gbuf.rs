use crate::{Color, Processor};

pub struct GBuf{
    buf: Vec<u32>,
    def_bg: u32,
    w: usize,
    h: usize
}

impl GBuf {

    pub fn new(w: usize, h: usize, c: u32) -> Self {
        
        let size: usize = w * h;
        let mut buf = Vec::with_capacity(size);
        buf.resize(size, c);

        Self {
            buf,
            def_bg: c,
            w,
            h
        }
    }

    pub fn resize(&mut self, w: usize, h: usize) {
        let new_len = w * h;
        if self.buf.len() != new_len {
            self.buf.resize(new_len, self.def_bg);
        }
        self.w = w;
        self.h = h;
    }

    pub fn resize_if_needed(&mut self, w: usize, h: usize) {
        if self.w != w || self.h != h {
            self.resize(w, h);
        }
    }

    #[inline]
    pub fn size(&self) -> (usize, usize) {
        (self.w, self.h)
    }

    pub fn flush(&mut self) {
        self.buf.fill(self.def_bg);
    }

    pub fn read(&self) -> &Vec<u32> {
        &self.buf
    }

    #[inline(always)]
    pub fn index(&self, x: usize, y: usize) -> Option<usize> {

        if x >= self.w || y >= self.h {

           return None;

        }

        Some(y * self.w + x)
    }

    #[inline(always)]
    pub fn set(&mut self, x: usize, y: usize, c: u32) {

        if let Some(i) = self.index(x, y) {
            self.buf[i] = c;
        }
    }

    #[inline(always)]
    pub fn blend(&mut self, x: usize, y: usize, c: u32) {

        if let Some(i) = self.index(x, y) {
            self.buf[i] = Color::blend_u32_to_u32(c, self.buf[i]);
        }
    }

    #[inline(always)]
    pub fn pixel(&self, x: usize, y: usize) -> Option<u32> {

        if let Some(i) = self.index(x, y) {

            return Some(self.buf[i])

        }

        None
    }

    #[inline(always)]
    pub fn merge(&mut self, x: usize, y: usize, (buf, w, h): (&Vec<u32>, usize, usize)) {

        let write_w = w.min(self.w.saturating_sub(x));
        let write_h = h.min(self.h.saturating_sub(y));

        if write_w == 0 || write_h == 0 {
            return; 
        }

        for j in 0..write_h {
            let row_start = j * w; 
            for i in 0..write_w {
                self.blend(x + i, y + j, buf[row_start + i]);
            }
        }
    }


    pub fn process<T>(&mut self, processor: T) 
    where T: Processor 
    {
        processor.process(&mut self.buf, self.w, self.h);
    }

}