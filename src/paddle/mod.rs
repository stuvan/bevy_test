use bevy::prelude::*;
use paddle_system::{paddle_movement, spawn_paddle};

pub mod paddle_component;
pub mod paddle_system;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_paddle);
        app.add_systems(Update, paddle_movement);
    }
}
