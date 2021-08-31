use gallium::{Component, EntityBuilder, EntityList, SceneBuilder, System, World, serde::{Serialize, Deserialize}};

/** A name component for an entity */
#[gallium::component]
pub struct Name(String);

/** The system that prints the name of an enetity */
#[derive(Serialize, Deserialize)]
pub struct TestSystem;

#[gallium::system]
impl System for TestSystem {
    fn tick(&self, scene: &mut gallium::Scene, world: &mut World) {
        // Get the entities with name components
        let entities = scene.get_entities()
        .are_active()
        .with_component::<Name>();

        // Loop over entities
        for entity in &entities {
            let name_component = entity.get_component::<Name>().unwrap();
            let name = &name_component.0;

            println!("Hello, {}:{}!", name, entity.id());
        }
    }
}


fn main() {
    // Create the world
    let mut world = World::new();    
    // Create the first scene
    let scene_a = SceneBuilder::new()
    .with_entity(EntityBuilder::new()
    .with(Name(String::from("Com")))
    .build())
    .with_system("test", TestSystem)
    .build();
    // Create the second scene
    let scene_b = SceneBuilder::new()
    .with_entity(EntityBuilder::new()
    .with(Name(String::from("Bob")))
    .build())
    .with_system("test", TestSystem)
    .build();
    
    // Set the scene to the first scene
    world.set_scene(scene_a);
    // Tick the systems in scene_a
    world.tick_systems("test");

    // Set the scene to the second scene
    world.set_scene(scene_b);
    // Tick the systems in scene_a
    world.tick_systems("test");
}