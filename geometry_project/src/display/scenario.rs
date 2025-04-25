use crate::Camera;
use minifb::Window;

pub trait IScenario {
    fn initialize(&mut self) -> Result<(), &'static str>;

    fn handle_input(&mut self, window: &Window);

    fn process(&mut self, camera: &mut Camera);

    fn redraw(&mut self) -> bool;
}
