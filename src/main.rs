mod bmp;
mod framebuffer;
mod pattern;

use framebuffer::FrameBuffer;
use minifb::{Key, Window, WindowOptions};
use pattern::Pattern;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let width = 500;
    let height = 500;
    let mut framebuffer = FrameBuffer::new(width, height);

    // Define patterns
    let patterns = vec![Pattern::new("Block", vec![(1, 1), (1, 2), (2, 1), (2, 2)])];

    // Draw each pattern scaled
    for pattern in &patterns {
        pattern.draw(&mut framebuffer);
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
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for &(dx, dy) in &directions {
        let nx = (x as isize + dx * 8) as usize;
        let ny = (y as isize + dy * 8) as usize;

        if nx < width && ny < height {
            let nidx = ny * width + nx;
            if framebuffer.buffer[nidx] == framebuffer.get_current_color() {
                count += 1;
            }
        }
    }

    count
}
