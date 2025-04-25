use log_statement::{def_log, def_log_data};

use crate::{
    algorithms::{convex_hull, random_geometry::Random2D},
    display::scenario::{self, IScenario},
    entities::{point2d::Point2d, rectangle2d::Rectangle2D},
};

pub struct ConvexHullScenario {
    pub count: usize,
    pub rect: Rectangle2D,
    points: Vec<Point2d>,
    new_pts: bool,
}

def_log!(ConvexHull);

impl ConvexHullScenario {
    pub fn new(count: usize, rect: Rectangle2D) -> Self {
        ConvexHullScenario {
            count,
            rect,
            points: Vec::with_capacity(count),
            new_pts: true,
        }
    }

    fn random_points(&mut self) {
        self.points.clear();
        self.points
            .extend(Random2D::random_points(self.rect, self.count as i32));

        convexhull_log!(
            "points: {}",
            serde_json::to_string(&self.points).ok().unwrap()
        );

        self.new_pts = true;
    }
}

impl IScenario for ConvexHullScenario {
    fn initialize(&mut self) -> Result<(), &'static str> {
        if self.count == 0 {
            return Err("Count must be greater than 0");
        }

        self.random_points();
        Ok(())
    }

    fn handle_input(&mut self, window: &minifb::Window) {
        if window.is_key_down(minifb::Key::R) {
            self.random_points();
        }
    }

    fn process(&mut self, camera: &mut crate::display::camera::Camera) {
        let polygon =
            convex_hull::convex_hull(&mut self.points).unwrap_or_else(|| panic!("Uhm didnt work"));

        camera.push_points(self.points.clone());

        camera.push_polygon(polygon);
    }

    fn redraw(&mut self) -> bool {
        if self.new_pts {
            self.new_pts = false;
            return true;
        }

        return false;
    }
}
