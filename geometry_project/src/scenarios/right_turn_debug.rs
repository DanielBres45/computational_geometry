use crate::{
    algorithms::{self, mixed_increment::MixedIncremenet, random_geometry::Random2D},
    display::{rgb::RGB, scenario::Scenario},
    entities::{line2d::Line2D, point2d::Point2d, rectangle2d::Rectangle2D},
};
use log_statement::def_log;

pub struct RightTurnDebug {
    pub count: usize,
    points: Vec<Point2d>,
    indexes: Vec<usize>,
    increment: MixedIncremenet,
    redraw: bool,
}

const LOGGING_ENABLED: bool = false;
def_log!(RightTurnDebug, LOGGING_ENABLED);

impl RightTurnDebug {
    pub fn new(count: usize, rect: Rectangle2D) -> Self {
        let points: Vec<Point2d> = Random2D::random_points(rect, count as i32)
            .into_iter()
            .collect();

        let mut debug = RightTurnDebug {
            count,
            points,
            indexes: Vec::new(),
            increment: MixedIncremenet::new_uniform(count, 3),
            redraw: true,
        };

        debug.increment();

        debug
    }

    fn increment(&mut self) {
        loop {
            match self.increment.next() {
                Some(val) => {
                    if val[0] == val[1] || val[1] == val[2] || val[0] == val[2] {
                        continue;
                    }

                    self.indexes = val;
                    return;
                }
                None => {
                    self.increment = MixedIncremenet::new_uniform(self.count, 3);
                }
            }
        }
    }

    fn decrement(&mut self) {
        loop {
            match self.increment.decrement() {
                Some(val) => {
                    if val[0] == val[1] || val[1] == val[2] || val[0] == val[2] {
                        continue;
                    }

                    self.indexes = val;
                    return;
                }
                None => {}
            }
        }
    }
}

impl Scenario for RightTurnDebug {
    fn initialize(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn handle_input(&mut self, window: &minifb::Window) {
        if window.is_key_pressed(minifb::Key::Right, minifb::KeyRepeat::No) {
            self.increment();
            self.redraw = true;
        } else if window.is_key_pressed(minifb::Key::Left, minifb::KeyRepeat::No) {
            self.decrement();
            self.redraw = true;
        } else if window.is_key_pressed(minifb::Key::Enter, minifb::KeyRepeat::No) {
            rightturndebug_log!(
                "a: {}, b: {}, c: {}",
                self.points[self.indexes[0]],
                self.points[self.indexes[1]],
                self.points[self.indexes[2]]
            );
        }
    }

    fn process(&mut self, camera: &mut crate::display::camera::Camera) {
        camera.push_points(self.points.clone());

        let a: Point2d = self.points[self.indexes[0]];
        let b: Point2d = self.points[self.indexes[1]];
        let c: Point2d = self.points[self.indexes[2]];

        camera.push_line(Line2D::new(a, b));
        camera.push_line(Line2D::new(b, c));

        let right_turn: bool = algorithms::convex_hull::right_turn(a, b, c);

        rightturndebug_log!(
            "Checking points: {}, {}, {}. Right turn: {} ",
            a,
            b,
            c,
            right_turn
        );

        let color: RGB = match right_turn {
            true => RGB::green(),
            false => RGB::red(),
        };

        camera.push_line_color(color);
        camera.push_line_color(color);
        self.redraw = false;
    }

    fn redraw(&mut self) -> bool {
        self.redraw
    }
}
