use ball_system::{
    ball_boundary_system, ball_paddle_collision_system, ball_speed_limiter_system, spawn_ball,
};
use bevy::prelude::*;

mod ball_component;
mod ball_system;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
        app.add_systems(
            Update,
            (
                ball_boundary_system,
                ball_paddle_collision_system,
                ball_speed_limiter_system,
            ),
        );
    }
}
