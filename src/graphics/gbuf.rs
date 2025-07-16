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
}