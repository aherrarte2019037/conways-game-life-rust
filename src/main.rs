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
    let patterns = vec![
        Pattern::new("Block", vec![(1, 1), (1, 2), (2, 1), (2, 2)]),
        Pattern::new(
            "Bee-hive",
            vec![(2, 1), (3, 1), (1, 2), (4, 2), (2, 3), (3, 3)],
        ),
        Pattern::new(
            "Loaf",
            vec![(2, 1), (3, 1), (1, 2), (4, 2), (2, 3), (4, 3), (3, 4)],
        ),
        Pattern::new("Boat", vec![(1, 1), (2, 1), (1, 2), (3, 2), (2, 3)]),
        Pattern::new("Tub", vec![(2, 1), (1, 2), (3, 2), (2, 3)]),
        Pattern::new("Blinker", vec![(2, 1), (2, 2), (2, 3)]),
        Pattern::new("Toad", vec![(2, 2), (3, 2), (4, 2), (1, 3), (2, 3), (3, 3)]),
        Pattern::new(
            "Beacon",
            vec![(1, 1), (2, 1), (1, 2), (4, 3), (3, 4), (4, 4)],
        ),
        Pattern::new(
            "Pulsar",
            vec![
                (4, 2),
                (5, 2),
                (6, 2),
                (10, 2),
                (11, 2),
                (12, 2),
                (2, 4),
                (2, 5),
                (2, 6),
                (7, 4),
                (7, 5),
                (7, 6),
                (9, 4),
                (9, 5),
                (9, 6),
                (14, 4),
                (14, 5),
                (14, 6),
                (4, 7),
                (5, 7),
                (6, 7),
                (10, 7),
                (11, 7),
                (12, 7),
                (4, 9),
                (5, 9),
                (6, 9),
                (10, 9),
                (11, 9),
                (12, 9),
                (2, 10),
                (2, 11),
                (2, 12),
                (7, 10),
                (7, 11),
                (7, 12),
                (9, 10),
                (9, 11),
                (9, 12),
                (14, 10),
                (14, 11),
                (14, 12),
                (4, 14),
                (5, 14),
                (6, 14),
                (10, 14),
                (11, 14),
                (12, 14),
            ],
        ),
        Pattern::new(
            "Penta-decathlon",
            vec![
                (2, 1),
                (2, 2),
                (2, 3),
                (2, 4),
                (2, 5),
                (2, 6),
                (2, 7),
                (2, 8),
                (2, 9),
                (2, 10),
                (2, 11),
                (2, 12),
                (2, 13),
                (2, 14),
                (2, 15),
            ],
        ),
        Pattern::new("Glider", vec![(1, 0), (2, 1), (0, 2), (1, 2), (2, 2)]),
        Pattern::new(
            "LWSS",
            vec![
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (0, 1),
                (4, 1),
                (4, 2),
                (0, 3),
                (3, 3),
            ],
        ),
        Pattern::new(
            "MWSS",
            vec![
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (0, 1),
                (4, 1),
                (4, 2),
                (0, 3),
                (3, 3),
                (1, 4),
            ],
        ),
        Pattern::new(
            "HWSS",
            vec![
                (1, 0),
                (2, 0),
                (3, 0),
                (4, 0),
                (5, 0),
                (0, 1),
                (5, 1),
                (5, 2),
                (0, 3),
                (4, 3),
                (2, 4),
            ],
        ),
    ];

    // Define fixed positions for the patterns
    let positions = vec![
        (50, 50),
        (50, height / 2 - 100),
        (width / 2 - 100, 50),
        (width / 2 - 100, height / 2 - 100),
        (width / 4, height / 4),
        (3 * width / 4 - 100, height / 4),
        (width / 4, 3 * height / 4 - 100),
        (3 * width / 4 - 100, 3 * height / 4 - 100),
        (width / 8, height / 8),
        (7 * width / 8 - 100, height / 8),
        (width / 8, 7 * height / 8 - 100),
        (7 * width / 8 - 100, 7 * height / 8 - 100),
        (3 * width / 8, 3 * height / 8),
        (5 * width / 8 - 100, 3 * height / 8),
        (3 * width / 8, 5 * height / 8 - 100),
        (5 * width / 8 - 100, 5 * height / 8 - 100),
    ];

    // Draw each pattern at specified positions
    for (pattern, &(offset_x, offset_y)) in patterns.iter().zip(positions.iter()) {
        pattern.draw(&mut framebuffer, offset_x, offset_y);
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

    for y in (0..height).step_by(8) {
        for x in (0..width).step_by(8) {
            // Kill cells on the edge
            if x == 0 || y == 0 || x + 8 >= width || y + 8 >= height {
                for i in 0..8 {
                    for j in 0..8 {
                        if x + i < width && y + j < height {
                            let idx = (y + j) * width + (x + i);
                            new_buffer[idx] = framebuffer.get_background_color();
                        }
                    }
                }
                continue;
            }

            let alive_neighbors = count_alive_neighbors(framebuffer, x, y);
            let idx = y * width + x;
            if framebuffer.buffer[idx] == framebuffer.get_current_color() {
                // Cell is alive
                if alive_neighbors < 2 || alive_neighbors > 3 {
                    new_buffer[idx] = framebuffer.get_background_color(); // Cell dies
                } else {
                    new_buffer[idx] = framebuffer.get_current_color(); // Cell survives
                }
            } else {
                // Cell is dead
                if alive_neighbors == 3 {
                    new_buffer[idx] = framebuffer.get_current_color(); // Cell becomes alive
                } else {
                    new_buffer[idx] = framebuffer.get_background_color(); // Cell remains dead
                }
            }
            // Copy new state to the entire block of 8x8 pixels
            for i in 0..8 {
                for j in 0..8 {
                    if x + i < width && y + j < height {
                        let block_idx = (y + j) * width + (x + i);
                        new_buffer[block_idx] = new_buffer[idx];
                    }
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
        (-8, -8),
        (0, -8),
        (8, -8),
        (-8, 0),
        (8, 0),
        (-8, 8),
        (0, 8),
        (8, 8),
    ];

    for &(dx, dy) in &directions {
        let nx = (x as isize + dx) as usize;
        let ny = (y as isize + dy) as usize;

        if nx < width && ny < height {
            let nidx = ny * width + nx;
            if framebuffer.buffer[nidx] == framebuffer.get_current_color() {
                count += 1;
            }
        }
    }

    count
}
