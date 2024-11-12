use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    InputManagerBundle,
};

use crate::{
    paddle::paddle_component::{Paddle, PaddleAction, PADDLE_SPEED, PADDLE_WIDTH},
    setup::collision::CustomCollison,
};

use super::paddle_component::PADDLE_HEIGHT;

pub fn spawn_paddle(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let bottom_padding = window.height() / 20.0;
    let paddle_y = -(window.height() / 2.0) + bottom_padding;

    let input_map = InputMap::new([
        (PaddleAction::LeftSlide, KeyCode::KeyA),
        (PaddleAction::RightSlide, KeyCode::KeyD),
        (PaddleAction::LeftSlide, KeyCode::ArrowLeft),
        (PaddleAction::RightSlide, KeyCode::ArrowRight),
    ]);

    commands.spawn((
        InputManagerBundle::<PaddleAction> {
            input_map,
            ..default()
        },
        RigidBody::Kinematic,
        Collider::rectangle(PADDLE_WIDTH, PADDLE_HEIGHT),
        SpriteBundle {
            transform: Transform::from_xyz(0.0, paddle_y, 0.8),
            texture: asset_server.load("png/paddleBlue.png"),
            ..default()
        },
        CollisionLayers::new([CustomCollison::Blue], [CustomCollison::Blue]),
        LinearVelocity::default(),
        Paddle {},
    ));
}

pub fn paddle_movement(
    query: Query<&ActionState<PaddleAction>, With<Paddle>>,
    mut velocity_query: Query<&mut LinearVelocity, With<Paddle>>,
    mut transform_query: Query<&mut Transform, With<Paddle>>,
    // time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let action_state = query.single();
    let window = window_query.get_single().unwrap();

    if let Ok(mut linear_velocity) = velocity_query.get_single_mut() {
        let mut direction = Vec2::ZERO;

        // Check for left or right movement inputs
        if action_state.pressed(&PaddleAction::LeftSlide) {
            direction.x -= 1.0;
        }
        if action_state.pressed(&PaddleAction::RightSlide) {
            direction.x += 1.0;
        }

        // Normalize direction if there is any movement
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        // Apply the velocity to the paddle (scaled by PADDLE_SPEED and delta time)
        linear_velocity.x = direction.x * PADDLE_SPEED;
        linear_velocity.y = 0.0; // Assuming no vertical movement

        // Now clamp the paddle's position based on the window size
        if let Ok(mut transform) = transform_query.get_single_mut() {
            // Get window dimensions
            let window_width = window.width();

            // Calculate the bounds based on the paddle's width
            let min_x = -window_width / 2.0 + PADDLE_WIDTH / 2.0;
            let max_x = window_width / 2.0 - PADDLE_WIDTH / 2.0;

            // Clamp the paddle's x position within the window bounds
            transform.translation.x = transform.translation.x.clamp(min_x, max_x);
        }
    }
}
