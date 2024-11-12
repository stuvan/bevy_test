use avian2d::prelude::{Collider, CollisionLayers};
use bevy::{prelude::*, window::PrimaryWindow};

use crate::setup::collision::CustomCollison;

use super::obstacle_component::Obstacle;
pub fn spawn_obstacles(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let num_rows = 5; // Number of rows of obstacles
    let num_columns = 10; // Number of columns of obstacles
    let obstacle_width = 64.0; // Width of each obstacle (adjust as needed)
    let obstacle_height = 32.0; // Height of each obstacle (adjust as needed)
    let spacing = 5.0; // Spacing between obstacles

    // Calculate the total width of the obstacle grid (for centering horizontally)
    let grid_width = (num_columns as f32) * (obstacle_width + spacing) - spacing; // Subtract spacing to avoid extra space on the far edge

    // Calculate the starting position for the first obstacle (centered horizontally)
    let start_x = -(grid_width / 2.0); // Center the grid horizontally
    let start_y = window.height() / 2.0 - 50.0; // Keep the vertical position fixed

    // Spawn the obstacles in a grid pattern
    for row in 0..num_rows {
        for col in 0..num_columns {
            // Calculate position for each obstacle
            let x = start_x + (col as f32) * (obstacle_width + spacing);
            let y = start_y - (row as f32) * (obstacle_height + spacing);

            commands.spawn((
                Obstacle, // Add the Obstacle component
                SpriteBundle {
                    transform: Transform::from_xyz(x, y, 0.0),
                    texture: asset_server.load("png/element_green_rectangle.png"), // Your obstacle texture
                    ..default()
                },
                CollisionLayers::new([CustomCollison::Blue], [CustomCollison::Blue]),
                Collider::rectangle(obstacle_width, obstacle_height), // Set collider shape for the obstacle
            ));
        }
    }
}
