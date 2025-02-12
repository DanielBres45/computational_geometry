use core::f32;

use crate::numerics::floating_comparisons::{approx_equal, approx_less};

use super::{line2d::Line2D, rectangle2d::Rectangle2D};

pub fn liang_barsky_clip(rect: Rectangle2D, line: Line2D) -> Option<Line2D> {
    let p1: f32 = -(line.end.x - line.start.x);
    let p2: f32 = -line.start.x;
    let p3: f32 = -(line.end.y - line.start.y);
    let p4: f32 = -p3;

    let q1: f32 = line.start.x - rect.min.x;
    let q2: f32 = rect.max.x - line.start.x;
    let q3: f32 = line.start.y - rect.min.x;
    let q4: f32 = rect.max.y - line.start.y;

    let mut posarr: [f32; 5] = [0f32; 5];
    let mut negarr: [f32; 5] = [0f32; 5];

    let posind: u8 = 0;
    let negind: u8 = 0;

    posarr[0] = 1f32;
    negarr[0] = 0f32;

    if (approx_equal(p1, 0f32, f32::EPSILON) && approx_less(q1, 0f32, f32::EPSILON))
        || (approx_equal(p2, 0f32, f32::EPSILON) && approx_less(q2, 0f32, f32::EPSILON))
        || (approx_equal(p3, 0f32, f32::EPSILON) && approx_less(q3, 0f32, f32::EPSILON))
        || (approx_equal(p4, 0f32, f32::EPSILON) & approx_less(q4, 0f32, f32::EPSILON))
    {
        return None;
    }

    None
}
