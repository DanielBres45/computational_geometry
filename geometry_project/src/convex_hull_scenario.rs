use crate::{
    algorithms::random_geometry::Random2D,
    display::scenario::{self, IScenario},
    entities::{point2d::Point2d, rectangle2d::Rectangle2D},
};

pub struct ConvexHullScenario {
    pub count: usize,
    pub rect: Rectangle2D,
    points: Vec<Point2d>,
    new_pts: bool,
}

impl ConvexHullScenario {
    pub fn new(count: usize, rect: Rectangle2D) -> Self {
        ConvexHullScenario {
            count,
            rect,
            points: Vec::with_capacity(count),
            new_pts: true,
        }
    }
}

impl IScenario for ConvexHullScenario {
    fn initialize(&mut self) -> Result<(), &'static str> {
        if self.count == 0 {
            return Err("Count must be greater than 0");
        }

        Ok(())
    }

    fn handle_input(&mut self, window: &minifb::Window) {
        if window.is_key_down(minifb::Key::R) {
            self.points.clear();

            self.points
                .extend(Random2D::random_point(self.rect, self.count));
        }
    }

    fn process(&mut self, camera: &mut crate::display::camera::Camera) {}
}
