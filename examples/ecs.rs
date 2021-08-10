use gallium_ecs::*;

// The component that stores a name
#[gallium::component]
struct NameComponent {
    name: String,
}

/** A system that greets the player */
#[derive(Serialize, Deserialize)]
struct GreetSystem;

#[gallium::system]
impl System for GreetSystem {
    /** Gets run each time the system is ticked */
    fn tick(&self, scene: &mut Scene) {
        // Fetch the entities
        let entities = scene
        .get_entities()
        .are_active()
        .with_component::<NameComponent>();
        
        // Loop over the entities
        for entity in entities.iter() {
            // Fetch the name component
            let name_component = entity.get_component::<NameComponent>().unwrap();
            // Greet the name
            println!("Hello, {}!", name_component.name);
        }
    }
}

/** Creates the scene and dumps it to a file */
pub fn create_scene() -> Scene {
    // Build the entity
    let entity1 = EntityBuilder::new()
        .with(NameComponent {
            name: String::from("Com"),
        })
        .build();

    let entity2 = EntityBuilder::new()
        .with(NameComponent {
        name: String::from("Bob")
        })
        .build();

    // Build the scene
    let scene = SceneBuilder::new()
        .with_entity(entity1)
        .with_entity(entity2)
        .with_system("init", GreetSystem {})
        .build();

    return scene;
}

fn main() {
    // Create the scene
    let mut scene = create_scene();

    // Tick the scenes init systems
    scene.tick_systems("init");
}
