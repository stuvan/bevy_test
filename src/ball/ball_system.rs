use crate::{
    obstacle::obstacle_component::Obstacle,
    paddle::paddle_component::{Paddle, PADDLE_HEIGHT, PADDLE_WIDTH},
    setup::collision::CustomCollison,
};
use avian2d::prelude::*;

use bevy::{prelude::*, window::PrimaryWindow};

use super::ball_component::{BlueBall, BALL_HEIGHT, BALL_WIDTH, MAX_BALL_SPEED};

pub fn spawn_ball(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let bottom_padding = window.height() / 20.0;
    let ball_y = (-(window.height() / 2.0) + bottom_padding) + PADDLE_HEIGHT;

    // Set the initial velocity (e.g., diagonal movement)
    let initial_velocity = Vec2::new(150.0, 150.0);

    commands.spawn((
        RigidBody::Dynamic,
        SpriteBundle {
            transform: Transform::from_xyz(0.0, ball_y, 0.8),
            texture: asset_server.load("png/ballBlue.png"),
            ..default()
        },
        Collider::circle(12.0),
        LinearVelocity {
            0: initial_velocity,
        },
        CollisionLayers::new([CustomCollison::Blue], [CustomCollison::Blue]),
        BlueBall {},
    ));
}

pub fn ball_speed_limiter_system(mut ball_query: Query<&mut LinearVelocity, With<BlueBall>>) {
    for mut velocity in ball_query.iter_mut() {
        // Calculate the speed (magnitude) of the velocity
        let speed = (velocity.x.powi(2) + velocity.y.powi(2)).sqrt(); // equivalent to Vec2.length()

        // If the ball's speed exceeds the max speed, clamp it
        if speed > MAX_BALL_SPEED {
            // Normalize the velocity and scale it to the max speed
            let scale = MAX_BALL_SPEED / speed;
            velocity.x *= scale;
            velocity.y *= scale;
        }
    }
}

pub fn ball_boundary_system(
    mut ball_query: Query<(&mut Transform, &mut LinearVelocity), With<BlueBall>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let window_width = window.width();
    let window_height = window.height();
    let ball_radius = 10.0; // Adjust according to the ball's size (if it's a circle, this should be half the diameter)

    for (mut transform, mut velocity) in ball_query.iter_mut() {
        // Horizontal boundary check (X axis)
        if transform.translation.x - ball_radius < -window_width / 2.0 {
            transform.translation.x = -window_width / 2.0 + ball_radius; // Clamp the position to the left edge
            velocity.x = velocity.x.abs(); // Reverse the X velocity to bounce right
        }
        if transform.translation.x + ball_radius > window_width / 2.0 {
            transform.translation.x = window_width / 2.0 - ball_radius; // Clamp the position to the right edge
            velocity.x = -velocity.x.abs(); // Reverse the X velocity to bounce left
        }

        // Vertical boundary check (Y axis)
        if transform.translation.y - ball_radius < -window_height / 2.0 {
            transform.translation.y = -window_height / 2.0 + ball_radius; // Clamp the position to the bottom edge
            velocity.y = velocity.y.abs(); // Reverse the Y velocity to bounce upwards
        }
        if transform.translation.y + ball_radius > window_height / 2.0 {
            transform.translation.y = window_height / 2.0 - ball_radius; // Clamp the position to the top edge
            velocity.y = -velocity.y.abs(); // Reverse the Y velocity to bounce downwards
        }
    }
}

pub fn ball_paddle_collision_system(
    mut ball_query: Query<(&mut Transform, &mut LinearVelocity), With<BlueBall>>,
    paddle_query: Query<&Transform, (With<Paddle>, Without<BlueBall>)>,
    // window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // let window = window_query.get_single().unwrap();
    // let window_width = window.width();
    // let window_height = window.height();
    let ball_radius = 10.0; // Adjust according to your ball's radius
    let paddle_width = PADDLE_WIDTH; // Set according to your paddle's width
    let paddle_height = PADDLE_HEIGHT; // Set according to your paddle's height

    for (mut ball_transform, mut ball_velocity) in ball_query.iter_mut() {
        for paddle_transform in paddle_query.iter() {
            let ball_position = ball_transform.translation;
            let paddle_position = paddle_transform.translation;

            // Check for collision between ball and paddle
            let distance_x = (ball_position.x - paddle_position.x).abs();
            let distance_y = (ball_position.y - paddle_position.y).abs();

            // If the ball is within the bounds of the paddle's hitbox, reverse its direction
            if distance_x < (paddle_width / 2.0 + ball_radius)
                && distance_y < (paddle_height / 2.0 + ball_radius)
            {
                // Ball collided with the paddle: Reverse the Y velocity and adjust the ball position
                ball_velocity.y = -ball_velocity.y.abs(); // Bounce upwards
                                                          // Adjust the ball's Y position to be on top of the paddle
                ball_transform.translation.y =
                    paddle_position.y + paddle_height / 2.0 + ball_radius;
            }
        }
    }
}
