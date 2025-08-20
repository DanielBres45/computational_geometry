use crate::entities::{line2d::Line2D, point2d::Point2d};

pub fn naive_line_intersection(lines: &Vec<Line2D>, epsilon: f32) -> Vec<Point2d> {
    if lines.len() <= 1 {
        return Vec::new();
    }

    let mut points: Vec<Point2d> = Vec::new();

    for i in 0..lines.len() - 1 {
        for j in i + 1..lines.len() {
            if let Some(p) = lines[i].intersect(&lines[j], epsilon) {
                points.push(p);
            }
        }
    }

    points
}
