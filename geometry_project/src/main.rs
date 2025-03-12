pub mod algorithms;
pub mod display;
pub mod entities;
pub mod extensions;
pub mod numerics;

use algorithms::random_geometry::Random2D;
use data_structures::vec2d::Vec2D;
use display::{camera::Camera, rgb::RGB};
use entities::{line2d::Line2D, point2d::Point2d, rectangle2d::Rectangle2D};
use log::{log, Level};
use logging::logger::logger::LoggingManager;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

pub const WINDOW_WIDTH: usize = 512;
pub const WINDOW_HEIGHT: usize = 512;

fn main() {
    LoggingManager::init("log_output.txt").expect("Failed to initialize Logger");

    let window = Window::new(
        "Geometry Renderer - Esc to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec2D<RGB> = Vec2D::new_from_flatpack(
        vec![RGB::black(); WINDOW_WIDTH * WINDOW_HEIGHT],
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .expect("width height unexpected");

    log!(Level::Info, "Lets start");

    window_loop(window, buffer);
}

fn window_loop(mut window: Window, mut buffer: Vec2D<RGB>) {
    let mut camera: Camera = Camera::new(100f32, 100f32);

    let min: Point2d = Point2d { x: 25f32, y: 25f32 };
    let max: Point2d = Point2d { x: 75f32, y: 75f32 };

    camera.push_points(Random2D::random_points(Rectangle2D { min, max }, 10));

    camera.push_line(Line2D::new(
        Point2d { x: 50f32, y: 25f32 },
        Point2d { x: 75f32, y: 75f32 },
    ));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(Duration::from_millis(1));

        camera.draw(&mut buffer);
        buffer_to_window(&mut window, buffer.clone());
    }
}

fn buffer_to_window(window: &mut Window, buffer2d: Vec2D<RGB>) {
    let buffer: Vec<u32> = buffer2d.into_iter().map(|r| r.value).collect();

    window.update_with_buffer(buffer.as_slice(), WINDOW_WIDTH, WINDOW_HEIGHT);
}
