use std::cell::{RefCell, RefMut};

use crate::{support::nameof, Scene};

/** Represents a world that manages scenes */
pub struct World {
    // The current scene
    scene: Option<Scene>,
    // The world resources
    resources: Vec<Box<dyn WorldResource>>,
}

impl World {
    /** Construct a new world */
    pub fn new() -> Self {
        return Self {
            scene: None,
            resources: vec![],
        };
    }

    /** Set the current scene */
    pub fn set_scene(&mut self, scene: Scene) {
        self.scene = Some(scene);
    }

    /** Insert a resource in the world */
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        // Make sure the resource doesn't already exist
        for r in &self.resources {
            if r.as_any().is::<T>() {
                panic!("World already has resource of type: {}", nameof(&resource));
            }
        }
        // Push the resource
        self.resources.push(Box::new(RefCell::new(resource)));
    }

    /** Get the resource from the world */
    pub fn get_resource<T: 'static>(&self) -> Option<RefMut<T>> {
        // Get the resource of the provided type
        for r in &self.resources {
            if let Some(r) = r.as_any().downcast_ref::<RefCell<T>>() {
                return Some(r.borrow_mut());
            }
        }
        // Return none if the resource hasn't been found
        return None;
    }

    /** Tick the systems in the current scene */
    pub fn tick_systems(&mut self, tag: &str) {
        // Take ownsership of the scene
        let mut scene = self.scene.take().expect("Scene not set in world.");
        // Tick the systems in the scene
        scene.tick_systems(tag, self);

        // Make sure the scene hasn't been changed
        if self.scene.is_none() {
            // Return ownsership of the scene
            self.scene = Some(scene);
        }
    }

    /** Dispatch an event over the the systems in the current scene */
    pub fn dispatch_event(&mut self, tag: &str, data: &dyn std::any::Any) {
        // Take ownsership of the scene
        let mut scene = self.scene.take().expect("Scene not set in world.");
        // Tick the systems in the scene
        scene.dispatch_event(tag, self, data);

        // Make sure the scene hasn't been changed
        if self.scene.is_none() {
            // Return ownsership of the scene
            self.scene = Some(scene);
        }
    }
}

/** Represents a world resource */
pub trait WorldResource {
    /** Return resource as any */
    fn as_any(&self) -> &dyn std::any::Any;
    /** Return resource as mutable any */
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/** Implement world_resource for any type */
impl<T: 'static> WorldResource for RefCell<T> {
    /** Return resource as any */
    fn as_any(&self) -> &dyn std::any::Any {
        return self;
    }

    /** Return resource as mutable any */
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        return self;
    }
}
