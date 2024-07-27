use crate::framebuffer::FrameBuffer;

struct Pattern {
    name: &'static str,
    coordinates: Vec<(usize, usize)>,
}

impl Pattern {
    fn new(name: &'static str, coordinates: Vec<(usize, usize)>) -> Self {
        Pattern { name, coordinates }
    }

    fn draw(&self, framebuffer: &mut FrameBuffer, scale: usize) {
        for &(x, y) in &self.coordinates {
            for i in 0..scale {
                for j in 0..scale {
                    framebuffer.point(x * scale + i, y * scale + j);
                }
            }
        }
    }
}