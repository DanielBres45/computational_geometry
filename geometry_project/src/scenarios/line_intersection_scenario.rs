use std::f32;

use crate::{
    algorithms::{self, random_geometry::Random2D},
    display::{camera::Camera, rgb::RGB, scenario::Scenario, scene::Scene},
    entities::{line2d::{Line2D, ParametricLine2D}, point2d::Point2d, rectangle2d::Rectangle2D},
};

use crate::scenarios::scenario_serializer;

use log_statement::def_log;
use serde::{Deserialize, Serialize};

pub struct LineIntersectionScenario {
    pub count: usize,
    pub rect: Rectangle2D,
    lines: Vec<Line2D>,
    round_points: bool,
    new_pts: bool,
}

const LOGGING_ENABLED: bool = true;
def_log!(LineIntersection, LOGGING_ENABLED);

const SCENARIO_FILE: &'static str = "line_intersection_scenario.txt";

impl LineIntersectionScenario {
    pub fn new(count: usize, rect: Rectangle2D) -> Self {
        LineIntersectionScenario {
            count,
            rect,
            lines: Vec::with_capacity(count),
            round_points: false,
            new_pts: true,
        }
    }

    pub fn round_points(mut self) -> Self {
        self.round_points = true;
        return self;
    }

    pub fn new_specific(lines: Vec<Line2D>, rect: Rectangle2D) -> Self {
        LineIntersectionScenario { count: lines.len(), rect, lines, round_points: false, new_pts: true }
    }

    fn save_scenario(&mut self) {
        scenario_serializer::save_entities_to_file(&self.lines, &SCENARIO_FILE, false);
    }

    fn load_scenario(&mut self) {
        self.lines = match scenario_serializer::deserialize_from_file(&SCENARIO_FILE) {
            Ok(l) => l,
            Err(s) => panic!("{}", s),
        };

        self.new_pts = true;
    }

    fn random_lines(&mut self) {
        self.lines.clear();

        if self.round_points
        {
            self.lines.extend(Random2D::random_lines_int(self.rect, self.count as i32));
        }
        else {
            self.lines.extend(Random2D::random_lines(self.rect, self.count as i32));
        }
        

        lineintersection_log!("{:?}", || Scene::from(&self.lines));

        self.new_pts = true;
    }
}

impl Scenario for LineIntersectionScenario {
    fn initialize(&mut self) -> Result<(), &'static str> {
        if self.count == 0 {
            return Err("Count must be greater than 0");
        }

        self.random_lines();
        Ok(())
    }

    fn handle_input(&mut self, window: &minifb::Window) {
        if window.is_key_pressed(minifb::Key::R, minifb::KeyRepeat::No) {
            self.random_lines();
        }

        if window.is_key_pressed(minifb::Key::S, minifb::KeyRepeat::No) {
            self.save_scenario();
        }

        if window.is_key_pressed(minifb::Key::L, minifb::KeyRepeat::No) {
            self.load_scenario();
        }
    }

    fn process(&mut self, camera: &mut crate::display::camera::Camera) {
        let intersections =
            algorithms::line_intersection::naive_line_intersection_with_lines(&self.lines, f32::EPSILON);
        
        camera.set_point_size(4);

        //println!("Lines: {:?}", &self.lines);
        println!("Points: {:?}", &intersections.iter().map(|t| -> String 
            {
                format!("Intersection: ({},{}), line1: {}, line2: {}", 
                t.0.x, t.0.y, ParametricLine2D(t.1), ParametricLine2D(t.2))
            }).collect::<Vec<String>>());

        for (point, segA, segB) in intersections
        {
            
        }

        //camera.push_points(intersections.iter().map(|f| -> Point2d { return f.0; }));

        //camera.set_point_color(RGB::green());
        //camera.push_lines(self.lines.clone());
    }

    fn redraw(&mut self) -> bool {
        if self.new_pts {
            self.new_pts = false;
            return true;
        }

        return false;
    }
}
