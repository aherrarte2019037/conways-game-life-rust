use crate::framebuffer::FrameBuffer;

pub struct Pattern {
    name: &'static str,
    coordinates: Vec<(usize, usize)>,
}

impl Pattern {
    pub fn new(name: &'static str, coordinates: Vec<(usize, usize)>) -> Self {
        Pattern { name, coordinates }
    }

    pub fn draw(&self, framebuffer: &mut FrameBuffer,) {
        for &(x, y) in &self.coordinates {
            for i in 0..8 {
                for j in 0..8 {
                    framebuffer.point(x * 8 + i, y * 8 + j);
                }
            }
        }
    }
}