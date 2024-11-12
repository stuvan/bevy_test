use bevy::prelude::*;
use obstacle_system::spawn_obstacles;

pub mod obstacle_component;
mod obstacle_system;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_obstacles);
        // app.add_systems(Update, paddle_movement);
    }
}
