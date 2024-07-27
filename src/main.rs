mod bmp;
mod framebuffer;
mod pattern;

use framebuffer::FrameBuffer;
use pattern::Pattern;
use std::thread::sleep;
use std::time::Duration;
use minifb::{Key, Window, WindowOptions};

fn main() {
    let width = 100;
    let height = 100;
    let mut framebuffer = FrameBuffer::new(width, height);

    // Define patterns
    let patterns = vec![
        Pattern::new("Block", vec![(1, 1), (1, 2), (2, 1), (2, 2)]),
        
    ];

    // Draw each pattern scaled
    let scale = 1; // Scaling factor
    for pattern in &patterns {
        pattern.draw(&mut framebuffer, scale);
    }

    let mut window = Window::new(
        "Conway's Game of Life",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        framebuffer.render_buffer_to_window(&mut window);
        next_generation(&mut framebuffer);
        window.update();
        sleep(Duration::from_millis(200));
    }

    println!("Simulation ended.");
}

fn next_generation(framebuffer: &mut FrameBuffer) {
    let mut new_buffer = framebuffer.buffer.clone();
    let width = framebuffer.width;
    let height = framebuffer.height;

    for y in 0..height {
        for x in 0..width {
            let alive_neighbors = count_alive_neighbors(framebuffer, x, y);
            let idx = y * width + x;
            if framebuffer.buffer[idx] == framebuffer.get_current_color() {
                // Cell is alive
                if alive_neighbors < 2 || alive_neighbors > 3 {
                    new_buffer[idx] = framebuffer.get_background_color(); // Cell dies
                }
            } else {
                // Cell is dead
                if alive_neighbors == 3 {
                    new_buffer[idx] = framebuffer.get_current_color(); // Cell becomes alive
                }
            }
        }
    }

    framebuffer.buffer = new_buffer;
}

fn count_alive_neighbors(framebuffer: &FrameBuffer, x: usize, y: usize) -> usize {
    let mut count = 0;
    let width = framebuffer.width;
    let height = framebuffer.height;
    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1, 0),         (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];

    for &(dx, dy) in &directions {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
            let nidx = ny as usize * width + nx as usize;
            if framebuffer.buffer[nidx] == framebuffer.get_current_color() {
                count += 1;
            }
        }
    }

    count
}