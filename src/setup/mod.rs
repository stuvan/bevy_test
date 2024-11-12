use crate::paddle::paddle_component::PaddleAction;
use bevy::prelude::*;
use camera_system::spawn_camera;
use leafwing_input_manager::prelude::*;

mod camera_system;
pub mod collision;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PaddleAction>::default());
        app.add_systems(Startup, spawn_camera);
    }
}
