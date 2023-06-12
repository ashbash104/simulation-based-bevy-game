use bevy::sprite::SpriteBundle;
pub struct WorkerAnt {
    name: String,
    sprite: SpriteBundle,
    position: (f32, f32),
    health: AntHealth, // or "Health" with bundle?
    inventory: f32,
    rizz: Rizz
}