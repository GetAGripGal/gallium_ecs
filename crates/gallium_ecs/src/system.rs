use crate::Scene;

/** To be inherited by each ecs system */
#[typetag::serde(tag = "system")]
pub trait System {
    /** Tick the system */
    fn tick(&self, _scene: &Scene) {}
    /** Handle game event */
    fn on_event(&self, _scene: &Scene, _tag: &str, _data: &dyn std::any::Any) {}
}
