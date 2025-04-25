use crate::{
    algorithms::random_geometry::Random2D,
    display::scenario::IScenario,
    entities::{point2d::Point2d, rectangle2d::Rectangle2D},
};
use log_statement::def_log;

pub struct RightTurnDebug {
    pub count: i32,
    points: Vec<Point2d>,
    i: usize,
    j: usize,
    k: usize,
}

def_log!(RightTurnDebug);

impl RightTurnDebug {
    pub fn new(rect: Rectangle2D) -> Self {
        let points: Vec<Point2d> = Random2D::random_points(rect, 10).into_iter().collect();

        RightTurnDebug {
            count: 10,
            points,
            i: 0,
            j: 1,
            k: 2,
        }
    }
}

impl IScenario for RightTurnDebug {
    fn initialize(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn handle_input(&mut self, window: &minifb::Window) {
        if window.is_key_down(minifb::Key::Right) {}
    }

    fn process(&mut self, camera: &mut crate::display::camera::Camera) {}

    fn redraw(&mut self) -> bool {
        true
    }
}
