const MIN_NORMAL: f32 = 1.19E-6f32;

//Adapated from floating-point-gui.de/errors/comparisons
pub fn approx_equal(a: f32, b: f32, epsilon: f32) -> bool {
    let abs_a: f32 = f32::abs(a);
    let abs_b: f32 = f32::abs(b);
    let diff: f32 = f32::abs(a - b);

    if a == b {
        true
    } else if a == 0f32 || b == 0f32 || (abs_a + abs_b < MIN_NORMAL) {
        diff < (epsilon * MIN_NORMAL)
    } else {
        diff / f32::min(abs_a + abs_b, f32::MAX) < epsilon
    }
}

pub fn approx_less(a: f32, b: f32, epsilon: f32) -> bool {
    if approx_equal(a, b, epsilon) {
        return false;
    }
    a < b
}

pub fn approx_equal_less(a: f32, b: f32, epsilon: f32) -> bool {
    if approx_equal(a, b, epsilon) {
        return true;
    }

    a < b
}

pub fn approx_greater(a: f32, b: f32, epsilon: f32) -> bool {
    if approx_equal(a, b, epsilon) {
        return false;
    }

    a > b
}

pub fn approx_equal_greater(a: f32, b: f32, epsilon: f32) -> bool {
    if approx_equal(a, b, epsilon) {
        return true;
    }

    a > b
}
