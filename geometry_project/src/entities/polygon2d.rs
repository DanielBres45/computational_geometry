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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_lines() {
        let polygon: Polygon2D = Polygon2D {
            points: [
                Point2d { x: 0f32, y: 0f32 },
                Point2d { x: 0f32, y: 10f32 },
                Point2d { x: 10f32, y: 10f32 },
                Point2d { x: 10f32, y: 0f32 },
            ]
            .into(),
        };

        let expected: Vec<Line2D> = [
            Line2D::new_flat(0f32, 0f32, 0f32, 10f32),
            Line2D::new_flat_int(0, 10, 10, 10),
            Line2D::new_flat_int(10, 10, 10, 0),
            Line2D::new_flat_int(10, 0, 0, 0),
        ]
        .into();

        let lines = polygon.lines();

        assert_eq!(expected.len(), lines.len());

        for i in 0..lines.len() {
            assert_eq!(expected[i], lines[i]);
        }
    }
}
