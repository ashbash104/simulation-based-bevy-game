use bevy::prelude::*;


struct Stockpile {
    food_count: u32,
}

struct MousePosition {
    position: Vec2,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(move_ants.system())
        .add_system(stockpile_inquiry.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn().insert(Ant {
        position: Vec2::new(0.0, 0.0),
        carrying_food: false,
    });
    commands.spawn().insert(Stockpile { food_count: 0 });
}

fn move_ants(
    mut query: Query<(&mut Ant, &mut Transform)>,
    stockpile_query: Query<&Transform, With<Stockpile>>,
) {
    for (mut ant, mut transform) in query.iter_mut() {
        if ant.carrying_food {
            let stockpile_transform = stockpile_query.single().unwrap();
            let target_position = Vec3::new(stockpile_transform.translation.x, stockpile_transform.translation.y, 0.0);
            let direction = (target_position.xy() - transform.translation.xy()).normalize();
            transform.translation += direction.extend(0.0) * 2.0;
            if (transform.translation.xy() - target_position.xy()).length() < 1.0 {
                ant.carrying_food = false;
            }
        } else {
            // Move the ant randomly
            let random_translation = Vec2::new(rand::random::<f32>() * 2.0 - 1.0, rand::random::<f32>() * 2.0 - 1.0);
            ant.position += random_translation * 0.1;
            transform.translation = ant.position.extend(0.0);
        }
    }
}

fn stockpile_inquiry(
    mouse_position: Res<MousePosition>,
    stockpile_query: Query<&Transform, With<Stockpile>>,
) {
    let stockpile_transform = stockpile_query.single().unwrap();
    let stockpile_position = Vec2::new(stockpile_transform.translation.x, stockpile_transform.translation.y);
    if (mouse_position.position - stockpile_position).length() < 1.0 {
        let stockpile = stockpile_query.single().unwrap();
        println!("Stockpile food count: {}", stockpile.food_count);
    }
}

/* This example sets up a basic Bevy application with an orthographic camera and creates an ant entity and a stockpile entity. The ant entity has a position and a boolean flag indicating whether it is carrying food. The stockpile entity has a food count. The MousePosition resource is used to track the position of the mouse cursor.

The move_ants system updates the position of the ants. If an ant is carrying food (ant.carrying_food == true), it moves towards the stockpile by adjusting its position. Once the ant reaches the stockpile, the carrying_food flag is set to false.

The stockpile_inquiry system checks if the mouse cursor is hovering over the stockpile square. If it is, it queries the stockpile entity to retrieve the food count and prints it to the console.

Please note that this is a simplified example for demonstration purposes and may not include all the necessary components and systems for a complete game. It should give you a starting point to build upon.*/