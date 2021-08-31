use crate::{Scene, World};

/** To be inherited by each ecs system */
#[typetag::serde(tag = "system")]
pub trait System {
    /** Tick the system */
    fn tick(&self, _scene: &mut Scene, _world: &mut World) {}
    /** Handle game event */
    fn on_event(
        &self,
        _scene: &mut Scene,
        _world: &mut World,
        _tag: &str,
        _data: &dyn std::any::Any,
    ) {
    }
}
