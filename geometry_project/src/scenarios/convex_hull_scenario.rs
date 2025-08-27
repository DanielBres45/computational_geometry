use log::debug;

use crate::{
    algorithms::{convex_hull, random_geometry::Random2D},
    display::scenario::Scenario,
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

    fn random_points(&mut self) {
        self.points.clear();
        self.points
            .extend(Random2D::random_points(self.rect, self.count as i32));

        debug!(
            "points: {}",
            serde_json::to_string(&self.points).ok().unwrap()
        );

        self.new_pts = true;
    }
}

impl Scenario for ConvexHullScenario {
    fn initialize(&mut self) -> Result<(), &'static str> {
        if self.count == 0 {
            return Err("Count must be greater than 0");
        }

        self.random_points();
        Ok(())
    }

    fn handle_input(&mut self, window: &minifb::Window) {
        if window.is_key_pressed(minifb::Key::R, minifb::KeyRepeat::No) {
            self.random_points();
        }
    }

    fn process(&mut self, camera: &mut crate::display::camera::Camera) {
        let polygon =
            convex_hull::convex_hull(&mut self.points).unwrap_or_else(|| panic!("Uhm didnt work"));

        camera.push_points(self.points.clone());
        camera.set_point_size(3);

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
