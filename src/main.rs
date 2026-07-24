mod framebuffer;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn draw_pattern(cells: &mut [Vec<bool>], center: Vector2, pattern: &[&str]) {
    let pattern_height = pattern.len() as isize;
    let pattern_width = pattern.iter().map(|row| row.len()).max().unwrap_or(0) as isize;
    let start_y = center.y.round() as isize - pattern_height / 2;
    let start_x = center.x.round() as isize - pattern_width / 2;

    for (row, pattern_row) in pattern.iter().enumerate() {
        for (column, symbol) in pattern_row.bytes().enumerate() {
            if symbol != b'O' {
                continue;
            }

            let y = start_y + row as isize;
            let x = start_x + column as isize;
            if y < 0 || x < 0 {
                continue;
            }

            if let Some(cell) = cells
                .get_mut(y as usize)
                .and_then(|grid_row| grid_row.get_mut(x as usize))
            {
                *cell = true;
            }
        }
    }
}

fn draw_pulsar(cells: &mut [Vec<bool>], center: Vector2) {
    draw_pattern(
        cells,
        center,
        &[
            "..OOO...OOO..",
            ".............",
            "O....O.O....O",
            "O....O.O....O",
            "O....O.O....O",
            "..OOO...OOO..",
            ".............",
            "..OOO...OOO..",
            "O....O.O....O",
            "O....O.O....O",
            "O....O.O....O",
            ".............",
            "..OOO...OOO..",
        ],
    );
}

fn draw_glider(cells: &mut [Vec<bool>], center: Vector2) {
    draw_pattern(cells, center, &[".O.", "..O", "OOO"]);
}

fn draw_lightweight_spaceship(cells: &mut [Vec<bool>], center: Vector2) {
    draw_pattern(cells, center, &["O.O.", "...O", "...O", "O..O", ".OOO"]);
}

fn draw_lightweight_spaceship_upwards(cells: &mut [Vec<bool>], center: Vector2) {
    draw_pattern(cells, center, &[".OOO", "O..O", "...O", "...O", "O.O."]);
}

fn draw_pentadecathlon(cells: &mut [Vec<bool>], center: Vector2) {
    draw_pattern(cells, center, &["..O....O..", "OO.OOOO.OO", "..O....O.."]);
}

fn draw_infinite(cells: &mut [Vec<bool>], center: Vector2) {
    draw_pattern(
        cells,
        center,
        &[
            "......O.", "....O.OO", "....O.O.", "....O...", "..O.....", "O.O.....",
        ],
    );
}

fn main() {
    let window_height = 800;
    let window_width = 800;
    let grid_width = 100usize;
    let grid_height = 100usize;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(grid_width as u32, grid_height as u32);

    let mut cells: Vec<Vec<bool>> = vec![vec![false; grid_width]; grid_height];
    let mut cells_buf: Vec<Vec<bool>> = vec![vec![false; grid_width]; grid_height];

    for i in 0..grid_width {
        draw_lightweight_spaceship(&mut cells, Vector2::new(i as f32 * 7.0, 8.0));
        draw_lightweight_spaceship_upwards(&mut cells, Vector2::new(i as f32 * 7.0, 70.0));
        draw_glider(&mut cells, Vector2::new(i as f32 * 5.0, 30.0));
        draw_glider(&mut cells, Vector2::new(i as f32 * 5.0, 60.0));
        draw_pentadecathlon(&mut cells, Vector2::new(i as f32 * 10.0, i as f32 * 10.0));
        draw_pentadecathlon(
            &mut cells,
            Vector2::new(100.0 - i as f32 * 10.0, 100.0 - i as f32 * 10.0),
        );
    }

    draw_pulsar(&mut cells, Vector2::new(50.0, 50.0));
    draw_pulsar(&mut cells, Vector2::new(95.0, 20.0));
    draw_pulsar(&mut cells, Vector2::new(80.0, 50.0));
    draw_pulsar(&mut cells, Vector2::new(80.0, 80.0));
    draw_pulsar(&mut cells, Vector2::new(20.0, 80.0));
    draw_infinite(&mut cells, Vector2::new(75.0, 50.0));
    draw_infinite(&mut cells, Vector2::new(25.0, 75.0));
    draw_infinite(&mut cells, Vector2::new(75.0, 50.0));
    draw_infinite(&mut cells, Vector2::new(25.0, 50.0));
    draw_infinite(&mut cells, Vector2::new(50.0, 80.0));

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
                framebuffer.set_pixel(j as u32, i as u32);
            }
        }

        std::mem::swap(&mut cells, &mut cells_buf);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        // 60 frames/sec ^-1 -> 0.0166 sec/frames -> 16.67 ms/frame
        thread::sleep(Duration::from_millis(100));
    }
}
