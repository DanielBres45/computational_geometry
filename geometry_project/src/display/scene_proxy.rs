use crate::{display::scene::Scene, entities::point2d::Point2d};

pub trait ISceneProxy {
    fn get_scene(&self) -> Scene;
}

impl ISceneProxy for Vec<Point2d> {
    fn get_scene(&self) -> Scene {
        let mut scene = Scene::new();
        scene.push_points(self.clone().into_iter());

        scene
    }
}

