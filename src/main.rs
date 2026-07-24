mod framebuffer;

use framebuffer::Framebuffer;
use raylib::prelude::*;

fn main() {
    let window_height = 600;
    let window_width = 800;
    let framebuffer_height = 400;
    let framebuffer_width = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    framebuffer.set_current_color(Color::RED);
    framebuffer.set_pixel(200, 200);

    while !window.window_should_close() {
        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
