use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    fs::File,
    io::Write,
};

use crate::{Component, Entity, System};
use ron::{
    de::from_reader,
    from_str,
    ser::{to_string_pretty, PrettyConfig},
    Error,
};
use serde::{Deserialize, Serialize};

/** Represents an ecs scene containing entities */
#[derive(Serialize, Deserialize)]
pub struct Scene {
    systems: Option<HashMap<String, Vec<Box<dyn System>>>>,
    entities: Vec<RefCell<Entity>>,
}

impl Scene {
    /** Construct a new scene */
    pub fn new() -> Self {
        return Self {
            systems: Some(HashMap::new()),
            entities: vec![],
        };
    }

    /** Add an entity to the scene */
    pub fn add_entity(&mut self, mut entity: Entity) {
        // Set the id
        entity.id = self.entities.len();
        // Add the entity
        self.entities.push(RefCell::new(entity));
    }

    /** Add a system to the scene */
    pub fn add_system<T: 'static + System>(mut self, tag: &str, system: T) {
        let systems = self.systems.as_mut().unwrap();

        // Add a system to the tag if the tag exists
        if systems.contains_key(tag) {
            systems.get_mut(tag).unwrap().push(Box::new(system));
            return;
        }
        // Create a tag and add the system
        systems.insert(String::from(tag), vec![Box::new(system)]);
    }

    /** Get an entity by its id */
    pub fn get_entity(&self, id: usize) -> Option<RefMut<Entity>> {
        if let Some(e) = self.entities.get(id) {
            return Some(e.borrow_mut());
        }
        return None;
    }

    /** Get a list of entities */
    pub fn get_entities(&self) -> Vec<RefMut<Entity>> {
        return self.entities.iter().map(|e| e.borrow_mut()).collect();
    }

    /** Tick the systems with specified tag in the scene */
    pub fn tick_systems(&mut self, tag: &str) {
        // Take ownership of the systems
        let mut systems = self.systems.take().unwrap();

        // Check if any system has specified tag
        if !systems.contains_key(tag) {
            println!("Scene doesnt include system with secified tag: {}", tag);
            return;
        }

        // Loop over systems
        for system in systems.get_mut(tag).unwrap().iter_mut() {
            system.tick(self);
        }

        // Return ownership of systems
        self.systems = Some(systems);
    }

    /** Dispatch a scene over the systems */
    pub fn dispatch_event(&mut self, tag: &str, data: &dyn std::any::Any) {
        // Take ownership of the systems
        let systems = self.systems.take().unwrap();

        // Loop over systems
        for system_list in systems.iter() {
            for system in system_list.1.iter() {
                // Handle events in system
                system.on_event(self, tag, data);
            }
        }

        // Return ownership of systems
        self.systems = Some(systems);
    }

    /** Serialize the scene to a ron string */
    pub fn to_ron(&self) -> Result<String, Error> {
        return to_string_pretty(&self, PrettyConfig::default());
    }

    /** Export scene to ron file */
    pub fn export_ron(&self, path: &str) -> std::io::Result<()> {
        // Create file
        let mut file = File::create(path)?;
        // Serialize to ron
        let ron = to_string_pretty(&self, PrettyConfig::default()).unwrap();
        // Dump to file
        file.write_all(ron.as_bytes())?;
        return Ok(());
    }

    /** Deserialize scene from ron */
    pub fn from_ron(ron: String) -> Result<Self, Error> {
        return from_str(&ron);
    }

    /** Import from a ron file */
    pub fn import_ron(path: &str) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let s: Scene = match from_reader(f) {
            Ok(s) => s,
            Err(e) => panic!("Failed to deserialize scene: {}", e),
        };
        return Ok(s);
    }
}

/** Builds the scene */
pub struct SceneBuilder {
    systems: HashMap<String, Vec<Box<dyn System>>>,
    entities: Vec<RefCell<Entity>>,
}

impl SceneBuilder {
    /** Builds a scene builder */
    pub fn new() -> Self {
        return Self {
            entities: vec![],
            systems: HashMap::new(),
        };
    }

    /** Add an entity to the scene */
    pub fn with_entity(mut self, mut entity: Entity) -> Self {
        // Set the id
        entity.id = self.entities.len();
        // Add the entity
        self.entities.push(RefCell::new(entity));
        return self;
    }

    /** Add a system to the scene */
    pub fn with_system<T: 'static + System>(mut self, tag: &str, system: T) -> Self {
        // Add a system to the tag if the tag exists
        if self.systems.contains_key(tag) {
            self.systems.get_mut(tag).unwrap().push(Box::new(system));
            return self;
        }
        // Create a tag and add the system
        self.systems
            .insert(String::from(tag), vec![Box::new(system)]);

        return self;
    }

    /** Build the scene */
    pub fn build(self) -> Scene {
        return Scene {
            entities: self.entities,
            systems: Some(self.systems),
        };
    }
}

/** A trait for entity list's */
pub trait EntityList {
    /** Remove entities that arent active from the list */
    fn are_active(self) -> Self;
    /** Remove entitites that dont have the provided component from the list */
    fn with_component<T: 'static + Component>(self) -> Self;
}

impl EntityList for Vec<RefMut<'_, Entity>> {
    /** Remove entities that arent active from the list */
    fn are_active(self) -> Self {
        return self
            .into_iter()
            .filter_map(move |e| if e.is_active { Some(e) } else { None })
            .collect();
    }

    /** Remove entitites that dont have the provided component from the list */
    fn with_component<T: 'static + Component>(self) -> Self {
        return self
            .into_iter()
            .filter_map(move |e| {
                if e.has_component::<T>() {
                    Some(e)
                } else {
                    None
                }
            })
            .collect();
    }
}
