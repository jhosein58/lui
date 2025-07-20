use crate::Processor;


pub struct BorderRadius {
    pub radius: usize
}

impl Processor for BorderRadius {
    fn process(&self, mut buffer: Vec<u32>, width: usize, height: usize) -> (Vec<u32>, usize, usize) {
        let radius = self.radius;
        for y in 0..height {
            for x in 0..width {
                let mut mask = true;

      
                 if x < radius && y < radius {
                    let dx = radius - x;
                    let dy = radius - y;
                    mask = dx*dx + dy*dy <= radius*radius;
                }

     
                else if x >= width - radius && y < radius {
                    let dx = x - (width - radius);
                    let dy = radius - y;
                    mask = dx*dx + dy*dy <= radius*radius;
                }


                else if x < radius && y >= height - radius {
                    let dx = radius - x;
                    let dy = y - (height - radius);
                    mask = dx*dx + dy*dy <= radius*radius;
                }


                else if x >= width - radius && y >= height - radius {
                    let dx = x - (width - radius);
                    let dy = y - (height - radius);
                    mask = dx*dx + dy*dy <= radius*radius;
                }

                if !mask {
                    buffer[y * width + x] = 0; 
                }
            }       
        }       

        (buffer, width, height)

    }
}