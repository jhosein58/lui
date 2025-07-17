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
    pub fn set(&mut self, x: usize, y: usize, c: u32) -> Result<(), ()> {

        if let Some(i) = self.index(x, y) {

            self.buf[i] = c;
            return Ok(());

        }

        Err(())
    }

    #[inline(always)]
    pub fn pixel(&self, x: usize, y: usize) -> Option<u32> {

        if let Some(i) = self.index(x, y) {

            return Some(self.buf[i])

        }

        None
    }

    #[inline(always)]
    pub fn merge(&mut self, x: usize, y: usize, buf: &[u32], w: usize, h: usize) -> Result<(), ()> {

        for j in 0..h {
            for i in 0..w{
                self.set(x + i, y + j, buf[j * w + i])?;
            }
        }
        
        Ok(())
    }

}