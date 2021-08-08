use std::cell::RefCell;

/** The trait to be inherited by any component */
#[typetag::serde(tag = "type")]
pub trait Component {
    /** Get component as any */
    fn as_any(&self) -> &dyn std::any::Any;
    /** Get component as mutable any */
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/** Represents a component storage */
#[typetag::serde(tag = "type")]
pub trait ComponentStorage {
    /** Get component as any */
    fn as_any(&self) -> &dyn std::any::Any;
    /** Get component as mutable any */
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

#[typetag::serde]
impl ComponentStorage for RefCell<Box<dyn Component>> {
    fn as_any(&self) -> &dyn std::any::Any {
        return self;
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        return self;
    }
}
