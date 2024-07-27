use minifb::Window;

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
    background_color: u32,
    current_color: u32,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        FrameBuffer {
            width,
            height,
            buffer: vec![0; width * height],
            background_color: 0x000000,
            current_color: 0xFFFFFF,
        }
    }

    pub fn render_buffer_to_window(&self, window: &mut Window) {
        window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = self.current_color;
        }
    }

    pub fn set_background_color(&mut self, color: u32) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = color;
    }

    pub fn get_background_color(&self) -> u32 {
        self.background_color
    }

    pub fn get_current_color(&self) -> u32 {
        self.current_color
    }
}
