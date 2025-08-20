use log::logger;

use crate::{
    display::{scene::Scene, scene_proxy::ISceneProxy},
    entities::{point2d::Point2d, polygon2d::Polygon2D, vect2d::Vector2D},
    extensions::vec_extensions::{VecExtensions, VecPointExtesions},
    numerics::floating_comparisons::{approx_greater, approx_less},
    scene_logger::scene_logger::SceneLogger,
};

use std::sync::OnceLock;

static LOGGER: OnceLock<SceneLogger> = OnceLock::new();

fn get_logger() -> &'static SceneLogger {
    LOGGER.get_or_init(|| SceneLogger::new("Algorithms", true))
}

fn log_scene<T>(proxy: &T)
where
    T: ISceneProxy,
{
    #[cfg(feature = "Algorithms")]
    {
        let logger = get_logger();
        logger.log_scene_proxy(proxy);
    }
}

pub fn right_turn(a: Point2d, b: Point2d, c: Point2d) -> bool {
    let v1: Vector2D = b - a;
    let v2: Vector2D = c - a;

    return approx_less(v1.cross(&v2), 0f32, 0.000001);
}

pub fn convex_hull(points: &mut Vec<Point2d>) -> Option<Polygon2D> {
    if points.len() < 3 {
        return None;
    }

    points.sort_lexicographic();

    let mut l_upper: Vec<Point2d> = Vec::new();
    l_upper.push(points[0]);
    l_upper.push(points[1]);

    for i in 2..points.len() {
        l_upper.push(points[i]);

        while l_upper.len() > 2 {
            if right_turn(
                l_upper.from_last(2),
                l_upper.from_last(1),
                l_upper.from_last(0),
            ) {
                break;
            }

            l_upper.remove(l_upper.len() - 2);
        }
    }

    log_scene(&l_upper);

    let mut l_lower: Vec<Point2d> = Vec::new();
    l_lower.push(points.from_last(0));
    l_lower.push(points.from_last(1));

    log_scene(&l_lower);

    for i in (0..points.len() - 3).rev() {
        l_lower.push(points[i]);

        log_scene(&l_lower);

        while l_lower.len() > 2 {
            if right_turn(
                l_lower.from_last(2),
                l_lower.from_last(1),
                l_lower.from_last(0),
            ) {
                break;
            }

            l_lower.remove(l_lower.len() - 2);
        }
    }

    l_lower.remove(l_lower.len() - 1);
    l_lower.remove(0);

    log_scene(&l_lower);

    l_upper.append(&mut l_lower);

    return Some(Polygon2D { points: l_upper });
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

    #[test]
    fn test_right_turn_3() {
        let pa: Point2d = Point2d::origin();
        let pb: Point2d = Point2d {
            x: -10f32,
            y: 10f32,
        };
        let pc: Point2d = Point2d { x: 10f32, y: 20f32 };

        assert!(right_turn(pa, pb, pc));
    }

    #[test]
    fn test_right_turn_line() {
        let pa: Point2d = Point2d { x: 75f32, y: 75f32 };
        let pb: Point2d = Point2d {
            x: 100f32,
            y: 100f32,
        };
        let pc: Point2d = Point2d {
            x: 125f32,
            y: 125f32,
        };

        assert!(!right_turn(pa, pb, pc));
    }

    #[test]
    fn test_basic() {
        let mut points: Vec<Point2d> = [
            Point2d { x: 0f32, y: 0f32 },
            Point2d { x: 0f32, y: 10f32 },
            Point2d { x: 10f32, y: 10f32 },
            Point2d { x: 10f32, y: 0f32 },
            Point2d { x: 5f32, y: 5f32 },
        ]
        .into();

        let hull = convex_hull(&mut points);
        assert!(matches!(hull, Some(_)));
    }
}
