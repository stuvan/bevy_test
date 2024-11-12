use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub const PADDLE_SPEED: f32 = 1000.0;
pub const PADDLE_WIDTH: f32 = 104.0;
#[allow(dead_code)]
pub const PADDLE_HEIGHT: f32 = 24.0;

#[derive(Component)]
pub struct Paddle;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PaddleAction {
    LeftSlide,
    RightSlide,
}
