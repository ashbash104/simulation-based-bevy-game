use bevy::sprite::SpriteBundle;
pub struct WorkerAnt {
    name: String,
    sprite: SpriteBundle,
    position: (f32, f32),
    health: AntHealth, 
    inventory: f32,
    rizz: Rizz
    base_speed: f32,
    weight_multiplier: f32,
}