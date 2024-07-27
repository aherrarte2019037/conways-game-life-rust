mod framebuffer;
mod bmp;
mod pattern;

use framebuffer::FrameBuffer;
use bmp::WriteBmp;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut framebuffer = FrameBuffer::new(500, 500); // Cambiar tamaño a 500x500

    // Define initial pattern (glider example)
    let initial_pattern = vec![
        (1, 0), (2, 1), (0, 2), (1, 2), (2, 2)
    ];

    // Draw the initial pattern scaled
    let scale = 8; // Scaling factor
    draw_scaled_pattern(&mut framebuffer, &initial_pattern, scale);

    // Number of iterations
    let iterations = 100;

    for _ in 0..iterations {
        framebuffer.render_buffer("out.bmp").unwrap();
        next_generation(&mut framebuffer);
        sleep(Duration::from_millis(200));
    }

    println!("Framebuffer rendered to output.bmp");
}

fn draw_scaled_pattern(framebuffer: &mut FrameBuffer, pattern: &Vec<(usize, usize)>, scale: usize) {
    for &(x, y) in pattern {
        for i in 0..scale {
            for j in 0..scale {
                framebuffer.point(x * scale + i, y * scale + j);
            }
        }
    }
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
