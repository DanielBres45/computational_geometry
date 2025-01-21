pub mod display;
pub mod entities;

use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

pub const WINDOW_WIDTH: usize = 512;
pub const WINDOW_HEIGHT: usize = 512;

fn main() {
    let window = Window::new(
        "Geometry Renderer - Esc to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let buffer: Vec<u32> = vec![0; WINDOW_WIDTH * WINDOW_HEIGHT];

    window_loop(window, buffer);
}

fn window_loop(mut window: Window, buffer: Vec<u32>) {
    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(Duration::from_millis(10));
        window.update_with_buffer(buffer.as_slice(), WINDOW_WIDTH, WINDOW_HEIGHT).expect("oops");
    }
}
