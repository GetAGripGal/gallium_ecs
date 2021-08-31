use ron::{
    de::from_reader,
    from_str,
    ser::{to_string_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};
use std::{
    cell::{RefCell, RefMut},
    fs::File,
    io::Write,
};

use crate::{support::nameof, Component};

/** Represents an entity with components */
#[derive(Serialize, Deserialize)]
pub struct Entity {
    // The entity id
    pub(crate) id: usize,
    // The entity is active
    pub is_active: bool,

    // Since we can't type-check a component if its already borrowed,
    // we need a differenc way of checking what components the entity has.
    // Right now i am just saving the names of the types to a vector and check whether the type-name is included
    stored_components: Vec<String>,

    // The entity components
    components: Vec<RefCell<Box<dyn Component>>>,
}

impl Entity {
    /** Import an entity from a ron file */
    pub fn import_ron(path: &str) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let e: Entity = match from_reader(file) {
            Ok(x) => x,
            Err(e) => panic!("Failed to deserialize entity: {}", e),
        };
        return Ok(e);
    }

    /** Create an entity from a ron string */
    pub fn from_ron(ron: &str) -> Self {
        let e: Entity = match from_str(ron) {
            Ok(x) => x,
            Err(e) => panic!("Failed to deserialize entity: {}", e),
        };
        return e;
    }

    /** Export entity to a ron file */
    pub fn export_ron(&self, path: &str) -> std::io::Result<()> {
        // Create file
        let mut file = File::create(path)?;
        // Serialize to ron
        let ron = self.to_ron();
        // Dump to file
        file.write_all(ron.as_bytes())?;
        return Ok(());
    }

    /** Serialize entity to a ron string */
    pub fn to_ron(&self) -> String {
        return match to_string_pretty(self, PrettyConfig::default()) {
            Ok(s) => s,
            Err(e) => panic!("Failed to serialize entity: {}", e),
        };
    }

    /** Get the entity id */
    pub fn id(&self) -> usize {
        return self.id;
    }

    /** Check if the entity has a component */
    pub fn has_component<T: 'static + Component>(&self) -> bool {
        let name = &String::from(std::any::type_name::<T>());
        return self.stored_components.contains(name);
    }

    /** Get the component of provided type */
    pub fn get_component<T: 'static + Component>(&self) -> Option<RefMut<T>> {
        // Loop over components
        for component in self.components.iter() {
            // Check if the component is already borrowed
            if let Ok(borrowed) = component.try_borrow() {
                // Check if the component is of type
                if !borrowed.as_any().is::<T>() {
                    continue;
                }
            // Continue if the component is already borrowed
            } else {
                continue;
            }
            // Return RefMut to the component
            return Some(RefMut::map(component.borrow_mut(), |component| {
                component.as_any_mut().downcast_mut::<T>().unwrap()
            }));
        }
        println!(
            "Component \'{}\' either doesn't exist in entity {} or is already borrowed",
            std::any::type_name::<T>(),
            self.id
        );
        return None;
    }
}

/** Builds an entity */
pub struct EntityBuilder {
    // The entity components
    components: Vec<RefCell<Box<dyn Component>>>,
    //
    stored_components: Vec<String>,
}

impl EntityBuilder {
    /** Construct entity builder */
    pub fn new() -> Self {
        return EntityBuilder {
            components: vec![],
            stored_components: vec![],
        };
    }

    /** Add component to the entity */
    pub fn with<T: 'static + Component>(mut self, component: T) -> Self {
        self.stored_components
            .push(String::from(nameof(&component)));
        self.components.push(RefCell::new(Box::new(component)));
        return self;
    }

    /** Build the entity */
    pub fn build(self) -> Entity {
        return Entity {
            id: 0,
            is_active: true,
            components: self.components,
            stored_components: self.stored_components,
        };
    }
}
