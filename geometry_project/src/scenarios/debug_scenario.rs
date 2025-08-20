use crate::display::scenario::Scenario;
use minifb::Window;

pub trait DebugScenario: Scenario {
    fn handle_debug_input(&mut self, window: &Window);
}
