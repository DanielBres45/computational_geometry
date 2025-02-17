use core::f32;

use crate::numerics::floating_comparisons::{approx_equal, approx_greater, approx_less};

use super::{line2d::Line2D, rectangle2d::Rectangle2D};

fn max_array(arr: Vec<f32>, n: usize) -> f32 {
    let mut max: f32 = f32::MIN;

    for i in 0..n {
        max = max.max(arr[i])
    }

    max
}

fn min_array(arr: Vec<f32>, n: usize) -> f32 {
    let mut min: f32 = f32::MAX;

    for i in 0..n {
        min = min.min(arr[i]);
    }

    min
}

pub fn liang_barsky_clip(rect: Rectangle2D, line: Line2D) -> Option<Line2D> {
    let p1: f32 = -(line.end.x - line.start.x);
    let p2: f32 = -line.start.x;
    let p3: f32 = -(line.end.y - line.start.y);
    let p4: f32 = -p3;

    let q1: f32 = line.start.x - rect.min.x;
    let q2: f32 = rect.max.x - line.start.x;
    let q3: f32 = line.start.y - rect.min.x;
    let q4: f32 = rect.max.y - line.start.y;

    let mut posarr: Vec<f32> = vec![0f32; 5];
    let mut negarr: Vec<f32> = vec![0f32; 5];

    let mut posind: u8 = 0;
    let mut negind: u8 = 0;

    posarr[0] = 1f32;
    negarr[0] = 0f32;

    if (approx_equal(p1, 0f32, f32::EPSILON) && approx_less(q1, 0f32, f32::EPSILON))
        || (approx_equal(p2, 0f32, f32::EPSILON) && approx_less(q2, 0f32, f32::EPSILON))
        || (approx_equal(p3, 0f32, f32::EPSILON) && approx_less(q3, 0f32, f32::EPSILON))
        || (approx_equal(p4, 0f32, f32::EPSILON) & approx_less(q4, 0f32, f32::EPSILON))
    {
        println!("Parallel to clipping window");
        return None; //parallel to clipping window
    }

    if !approx_equal(p1, 0f32, f32::EPSILON) {
        let r1: f32 = q1 / p1;
        let r2: f32 = q2 / p2;

        if approx_less(p1, 0f32, f32::EPSILON) {
            negarr[negind as usize] = r1;
            posarr[posind as usize] = r2;
        } else {
            negarr[negind as usize] = r1;
            posarr[posind as usize] = r1;
        }

        negind += 1;
        posind += 1;
    }

    if !approx_equal(p3, 0f32, f32::EPSILON) {
        let r3: f32 = q3 / p3;
        let r4: f32 = q4 / p4;

        if approx_less(p3, 0f32, f32::EPSILON) {
            negarr[negind as usize] = r3;
            posarr[posind as usize] = r4;
        } else {
            negarr[negind as usize] = r4;
            posarr[posind as usize] = r3;
        }
    }

    let rn1: f32 = max_array(negarr, negind as usize);
    let rn2: f32 = min_array(posarr, posind as usize);

    if approx_greater(rn1, rn2, f32::EPSILON) {
        return None; //outisde clipping window
    }

    let xn1: f32 = line.start.x + p2 * rn1;
    let yn1: f32 = line.start.y + p4 * rn1;

    let xn2: f32 = line.start.x + p2 * rn2;
    let yn2: f32 = line.start.y + p4 * rn2;

    Some(Line2D::new_flat(xn1, yn1, xn2, yn2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_outside() {
        let rect: Rectangle2D = Rectangle2D::new_width_height(10f32, 10f32);
        let line: Line2D = Line2D::new_flat(0f32, 20f32, 20f32, 20f32);

        let clipped = liang_barsky_clip(rect, line);

        assert!(clipped.is_none(), "Line is outside rect");
    }

    #[test]
    fn test_inside_1() {
        let rect: Rectangle2D = Rectangle2D::new_width_height(100f32, 100f32);
        let line: Line2D = Line2D::new_flat(10f32, 10f32, 90f32, 90f32);

        let clipped = liang_barsky_clip(rect, line);
        assert!(clipped.is_some());

        assert!(clipped.unwrap().approx_equals(&line, f32::EPSILON));
    }
}
