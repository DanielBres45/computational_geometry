use std::{cmp::Ordering, f32};

use crate::{
    entities::{point2d::Point2d, polygon2d::Polygon2D, vect2d::Vector2D},
    extensions::vec_extensions::{VecExtensions, VecPointExtesions},
    numerics::floating_comparisons::{approx_greater, approx_less},
};

fn right_turn(a: Point2d, b: Point2d, c: Point2d) -> bool {
    let v1: Vector2D = b - a;
    let v2: Vector2D = c - b;

    return approx_less(v1.cross(&v2), 0f32, 0.0001);
}

pub fn convex_hull(points: &mut Vec<Point2d>) -> Option<Polygon2D> {
    if points.len() < 3 {
        return None;
    }

    points.sort_lexicographic();

    let mut l_upper: Vec<Point2d> = Vec::new();
    l_upper.push(points[0]);
    l_upper.push(points[1]);

    for i in 3..points.len() {
        l_upper.push(points[i]);

        while l_upper.len() > 2 {
            if !right_turn(
                l_upper.from_last(2),
                l_upper.from_last(1),
                l_upper.from_last(0),
            ) {
                break;
            }

            l_upper.remove(l_upper.len() - 2);
        }
    }

    let mut l_lower: Vec<Point2d> = Vec::new();
    l_lower.push(points.from_last(0));
    l_lower.push(points.from_last(1));

    for i in points.len() - 3..0 {
        l_lower.push(points[i]);

        while l_lower.len() > 2 {
            if !right_turn(
                l_lower.from_last(2),
                l_lower.from_last(1),
                l_lower.from_last(0),
            ) {
                break;
            }

            l_lower.remove(l_lower.len() - 2);
        }
    }

    l_lower.append(&mut l_upper);

    return Some(Polygon2D { points: l_lower });
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_right_turn_1() {
        let pa: Point2d = Point2d { x: 0f32, y: 0f32 };
        let pb: Point2d = Point2d { x: 0f32, y: 10f32 };
        let pc: Point2d = Point2d { x: 10f32, y: 10f32 };

        assert!(right_turn(pa, pb, pc));
    }

    #[test]
    fn test_right_turn_2() {
        let pa: Point2d = Point2d::origin();
        let pb: Point2d = Point2d { x: 0f32, y: 10f32 };
        let pc: Point2d = Point2d {
            x: -10f32,
            y: 10f32,
        };

        assert!(!right_turn(pa, pb, pc));
    }
}
