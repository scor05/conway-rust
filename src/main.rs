mod framebuffer;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn main() {
    let window_height = 100;
    let window_width = 100;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    let grid_width = window_width as usize;
    let grid_height = window_height as usize;

    let mut cells: Vec<Vec<bool>> = vec![vec![false; grid_width]; grid_height];
    let mut cells_buf: Vec<Vec<bool>> = vec![vec![false; grid_width]; grid_height];

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_S) {
            framebuffer.render_to_file("current_frame.png");
        }

        for i in 1..grid_height - 1 {
            for j in 1..grid_width - 1 {
                let mut alive_neighbors = 0;
                for k in (i - 1)..=(i + 1) {
                    for l in (j - 1)..=(j + 1) {
                        if k == i && l == j {
                            continue;
                        }
                        if cells[k][l] {
                            alive_neighbors += 1;
                        }
                    }
                }

                cells_buf[i][j] = alive_neighbors == 3 || (cells[i][j] && alive_neighbors == 2);

                if cells[i][j] {
                    framebuffer.set_current_color(Color::RED);
                } else {
                    framebuffer.set_current_color(Color::BLACK);
                }
                framebuffer.set_pixel(i as u32, j as u32);
            }
        }

        std::mem::swap(&mut cells, &mut cells_buf);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        // 60 frames/sec ^-1 -> 0.0166 sec/frames -> 16.67 ms/frame
        thread::sleep(Duration::from_millis(250));
    }
}
