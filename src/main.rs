mod framebuffer;
mod bmp;

use framebuffer::FrameBuffer;
use bmp::WriteBmp;

fn main() {
    let mut framebuffer = FrameBuffer::new(400, 400);

    framebuffer.set_background_color(0xeb4034);
    
    framebuffer.set_current_color(0xeb4034);
    framebuffer.point(400, 300);
    framebuffer.point(401, 300);
    framebuffer.point(400, 301);
    framebuffer.point(401, 301);

    framebuffer.set_current_color(0xeb4034);
    framebuffer.point(200, 150);
    framebuffer.point(201, 150);
    framebuffer.point(200, 151);
    framebuffer.point(201, 151);

    framebuffer.set_current_color(0xeb4034);
    framebuffer.point(600, 450);
    framebuffer.point(601, 450);
    framebuffer.point(600, 451);
    framebuffer.point(601, 451);

    framebuffer.render_buffer("out.bmp").unwrap();

    println!("Framebuffer rendered to output.bmp");
}