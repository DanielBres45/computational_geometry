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

    let specific_scenario: Vec<Line2D> = 
    vec![Line2D { start: Point2d { x: 73.470276, y: 64.859955 }, end: Point2d { x: 46.834213, y: 38.314342 } }, 
    Line2D { start: Point2d { x: 62.55593, y: 73.8566 }, end: Point2d { x: 54.35093, y: 58.3027 } }, 
    Line2D { start: Point2d { x: 27.983957, y: 52.657627 }, end: Point2d { x: 45.616966, y: 26.074202 } }, 
    Line2D { start: Point2d { x: 57.50478, y: 34.425934 }, end: Point2d { x: 50.000343, y: 34.431786 } }, 
    Line2D { start: Point2d { x: 56.74343, y: 49.15623 }, end: Point2d { x: 39.79673, y: 46.38095 } }, 
    Line2D { start: Point2d { x: 56.412815, y: 32.000595 }, end: Point2d { x: 27.73239, y: 61.217308 } }, 
    Line2D { start: Point2d { x: 29.940807, y: 59.32866 }, end: Point2d { x: 57.583504, y: 49.419746 } }, 
    Line2D { start: Point2d { x: 73.244606, y: 28.111584 }, end: Point2d { x: 71.43653, y: 29.928463 } }, 
    Line2D { start: Point2d { x: 62.15374, y: 72.334 }, end: Point2d { x: 28.37599, y: 51.491726 } }, 
    Line2D { start: Point2d { x: 68.03107, y: 42.681534 }, end: Point2d { x: 66.35208, y: 45.307434 } }];

    let scenario: &mut dyn Scenario =
         &mut LineIntersectionScenario::new_specific(specific_scenario, Rectangle2D { min, max });

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
    let buffer: Vec<u32> = buffer2d.into_iter().map(|r| r.value).collect();

    window.update_with_buffer(buffer.as_slice(), WINDOW_WIDTH, WINDOW_HEIGHT);
}
