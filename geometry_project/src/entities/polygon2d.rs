use super::{line2d::Line2D, point2d::Point2d};

pub struct Polygon2D {
    pub points: Vec<Point2d>,
}

impl Polygon2D {
    pub fn lines(&self) -> Vec<Line2D> {
        let mut lines: Vec<Line2D> = Vec::new();

        let max_pts: usize = self.points.len();

        for i in 1..max_pts {
            lines.push(Line2D {
                start: self.points[i - 1],
                end: self.points[i],
            });
        }

        lines.push(Line2D {
            start: self.points[max_pts - 1],
            end: self.points[0],
        });

        lines
    }
}
