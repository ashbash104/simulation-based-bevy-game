use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
mod worker_ant_struct;
use worker_ant_struct::WorkerAnt;
impl WorkerAnt {
    fn new(base_speed: f32, weight_multiplier: f32, health: u32) -> Self {
        WorkerAnt {
            base_speed,
            weight_multiplier,
            health,
        }
    }

    fn pick_up_food(&mut self) {
        self.weight_multiplier += 0.1; // Adjust the weight multiplier as needed
    }
    fn query_health(&self) -> u32 {
        self.health
    }
}