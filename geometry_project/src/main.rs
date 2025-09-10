pub mod algorithms;
pub mod display;
pub mod entities;
pub mod extensions;
pub mod numerics;
mod scenarios;
pub mod scene_logger;
mod testing_tools;

use ctrlc::set_handler;
use data_structures::vec2d::Vec2D;
use display::{camera::Camera, rgb::RGB, scenario::Scenario};
use entities::{line2d::Line2D, point2d::Point2d, rectangle2d::Rectangle2D};
use log::{log, Level, Log, trace};
use log_statement::def_log;
use logging::flush;
use logging::logger::logger::LoggingManager;
use minifb::{Key, Window, WindowOptions};
use scenarios::{
    convex_hull_scenario::ConvexHullScenario, line_intersection_scenario::LineIntersectionScenario,
    right_turn_debug::RightTurnDebug,
};
use std::time::Duration;

pub const WINDOW_WIDTH: usize = 512;
pub const WINDOW_HEIGHT: usize = 512;

fn main() {
    LoggingManager::init("log_output.txt").expect("Failed to initialize Logger");

    set_handler(|| handle_sigint()).expect("Error setting ctrlc hook");

    let window = Window::new(
        "Geometry Renderer - Esc to exit",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let buffer = blank_screen();

    log!(Level::Info, "Lets start");

    window_loop(window, buffer);
}

fn handle_sigint() {
    eprintln!("Caught Ctrl^C flushing log");
    LoggingManager::static_flush();
    std::process::exit(1);
}

fn blank_screen() -> Vec2D<RGB> {
    Vec2D::new_from_flatpack(
        vec![RGB::black(); WINDOW_WIDTH * WINDOW_HEIGHT],
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
    )
    .expect("width height unexpected")
}

fn window_loop(mut window: Window, mut buffer: Vec2D<RGB>) {
    let mut camera: Camera = Camera::new(100f32, 100f32);

    let min: Point2d = Point2d { x: 25f32, y: 25f32 };
    let max: Point2d = Point2d { x: 75f32, y: 75f32 };

    let scenario: &mut dyn Scenario =
         &mut LineIntersectionScenario::new_first_intersection(2, Rectangle2D { min, max })
         .round_points();

    //let scenario: &mut dyn Scenario = &mut ConvexHullScenario::new(10, Rectangle2D { min, max });

    match scenario.initialize() {
        Ok(_) => {}
        Err(e_msg) => {
            panic!("{}", e_msg);
        }
    };

    println!("Ready to render");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        std::thread::sleep(Duration::from_millis(5));

        trace!("next frame");
        scenario.handle_input(&window);

        if scenario.redraw() {
            trace!("Redraw scenario");
            camera.clear();
            buffer = blank_screen();
            scenario.process(&mut camera);

            camera.draw(&mut buffer);
        }

        buffer_to_window(&mut window, buffer.clone());
    }

    flush!("end");
}

fn buffer_to_window(window: &mut Window, buffer2d: Vec2D<RGB>) {
    let buffer: Vec<u32> = buffer2d.into_iter().map(|r| r.0).collect();

    window.update_with_buffer(buffer.as_slice(), WINDOW_WIDTH, WINDOW_HEIGHT);
}
