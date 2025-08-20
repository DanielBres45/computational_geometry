use std::panic::Location;

use crate::display::scene::Scene;
use crate::display::scene_proxy::ISceneProxy;

pub struct SceneLogger 
{
    target: &'static str,
    enabled: bool
}

impl SceneLogger {
   
    pub fn new(target: &'static str, enabled: bool) -> Self {

        Self {
            target,
            enabled
        }

    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn log_scene(&self, scene: Scene, location: &Location) -> bool
    {
        if !self.enabled
        {
            return false;
        }

        let serialized = match serde_json::to_string(&scene)
        {
            Ok(val) => val,
            Err(_) => {return false;}
        };

        log::debug!(
            target: &self.target, 
            " file_name = {}, line_number = {}, scene_data = {}",
            location.file(), location.line(), serialized);

        true
    }

    #[track_caller]
    pub fn log_scene_proxy<T: ISceneProxy + Sized>(&self, to_scene: &T) -> bool {
        if !self.enabled
        {
            return false;
        }

        return self.log_scene(to_scene.get_scene(), Location::caller());
    }

    #[track_caller]
    pub fn log_scene_proxy_closure<F,T>(&self, caller: F) -> bool 
        where 
            F : FnOnce() -> T,
            T : ISceneProxy
    {
        let proxy: T = caller();

        return self.log_scene(proxy.get_scene(), Location::caller());
    }
}

