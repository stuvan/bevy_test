use avian2d::{math::*, prelude::*};
use bevy::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum CustomCollison {
    #[default]
    Default,
    Blue,
    Red,
}
