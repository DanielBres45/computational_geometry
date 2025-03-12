use std::cmp::Ordering;

use rand::seq::IndexedRandom;

use crate::{
    entities::{point2d::Point2d, polygon2d::Polygon2D, vect2d::Vector2D},
    extensions::vec_extensions::{VecExtensions, VecPointExtesions},
    numerics::floating_comparisons::approx_greater,
};

pub fn convex_hull(mut points: Vec<Point2d>) -> Option<Polygon2D> {
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
            let v1: Vector2D = l_upper.from_last(1) - l_upper.from_last(2);
            let v2: Vector2D = l_upper.from_last(0) - l_upper.from_last(1);

            if approx_greater(v1.cross(&v2), 0f32, f32::EPSILON) {
                break;
            }

            l_upper.remove(l_upper.len() - 2);
        }
    }

    return None;
}
