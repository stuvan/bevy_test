use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{color::palettes::tailwind::SKY_200, prelude::*};
use leafwing_input_manager::prelude::ActionState;

use crate::paddle::paddle_component::{Paddle, PaddleAction, PADDLE_SPEED, PADDLE_WIDTH};

#[derive(Component)]
pub struct Buttons;

#[derive(Component)]
pub struct LButton;

#[derive(Component)]
pub struct RButton;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_button);
        app.add_systems(
            Update,
            (handle_button_click_right, handle_button_click_left),
        );
    }
}

pub fn build_button(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let button_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                // background_color: BackgroundColor(Color::srgb(1.0, 0.0, 0.0)),
                ..default()
            },
            Buttons,
        ))
        .with_children(|parent| {
            // Left
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(4.0)),
                            width: Val::Px(80.),
                            height: Val::Px(40.),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::srgb(0., 1., 0.)),
                        ..Default::default()
                    },
                    LButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Left",
                                TextStyle {
                                    // font:
                                    ..Default::default()
                                },
                            )],
                            justify: JustifyText::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            // Right
        })
        .with_children(|parent| {
            // Left
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            padding: UiRect::all(Val::Px(4.0)),
                            width: Val::Px(80.),
                            height: Val::Px(40.),
                            ..Default::default()
                        },
                        background_color: BackgroundColor(Color::srgb(0., 1., 0.)),
                        ..Default::default()
                    },
                    RButton {},
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Right",
                                TextStyle {
                                    // font:
                                    ..Default::default()
                                },
                            )],
                            justify: JustifyText::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            // Right
        })
        .id();

    button_entity
}

pub fn spawn_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_button(&mut commands, &asset_server);
}

pub fn handle_button_click_left(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<LButton>),
    >,
    query: Query<&ActionState<PaddleAction>, With<Paddle>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut velocity_query: Query<&mut LinearVelocity, With<Paddle>>,
    mut transform_query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>, // Time resource to make movement frame-rate independent
) {
    let action_state = query.single();
    let window = window_query.get_single().unwrap();

    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // When the button is first pressed, apply velocity to start moving
                if let Ok(mut linear_velocity) = velocity_query.get_single_mut() {
                    let mut direction = Vec2::ZERO;
                    direction.x -= 1.0; // Move left (negative X direction)
                    if direction.length() > 0.0 {
                        direction = direction.normalize();
                    }

                    // Apply the velocity to the paddle (scaled by PADDLE_SPEED)
                    linear_velocity.x = direction.x * PADDLE_SPEED;
                    linear_velocity.y = 0.0; // No vertical movement for the paddle
                }
            }
            Interaction::Hovered => {
                // While the button is hovered (pressed or held), continue moving the paddle
                if let Ok(mut linear_velocity) = velocity_query.get_single_mut() {
                    let mut direction = Vec2::ZERO;
                    direction.x -= 1.0; // Move left (negative X direction)
                    if direction.length() > 0.0 {
                        direction = direction.normalize();
                    }

                    // Keep applying the velocity to the paddle
                    linear_velocity.x = direction.x * PADDLE_SPEED;
                    linear_velocity.y = 0.0; // No vertical movement for the paddle
                }
            }
            // The button is no longer being pressed or hovered (button is released)
            Interaction::None => {
                // When the button is no longer interacted with, stop the paddle movement
                if let Ok(mut linear_velocity) = velocity_query.get_single_mut() {
                    linear_velocity.x = 0.0;
                    linear_velocity.y = 0.0;
                }
            }
            _ => (),
        }
    }

    // Clamping the paddle's position based on the window size (avoiding movement out of bounds)
    if let Ok(mut transform) = transform_query.get_single_mut() {
        let window_width = window.width();
        let min_x = -window_width / 2.0 + PADDLE_WIDTH / 2.0;
        let max_x = window_width / 2.0 - PADDLE_WIDTH / 2.0;
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    }
}

pub fn handle_button_click_right(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<RButton>),
    >,
    query: Query<&ActionState<PaddleAction>, With<Paddle>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut velocity_query: Query<&mut LinearVelocity, With<Paddle>>,
    mut transform_query: Query<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    let action_state = query.single();
    let window = window_query.get_single().unwrap();

    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                // When the button is first pressed, apply velocity to start moving
                if let Ok(mut linear_velocity) = velocity_query.get_single_mut() {
                    let mut direction = Vec2::ZERO;
                    direction.x += 1.0; //
                    if direction.length() > 0.0 {
                        direction = direction.normalize();
                    }

                    // Apply the velocity to the paddle (scaled by PADDLE_SPEED)
                    linear_velocity.x = direction.x * PADDLE_SPEED;
                    linear_velocity.y = 0.0; // No vertical movement for the paddle
                }
            }
            Interaction::Hovered => {
                // While the button is hovered (pressed or held), continue moving the paddle
                if let Ok(mut linear_velocity) = velocity_query.get_single_mut() {
                    let mut direction = Vec2::ZERO;
                    direction.x += 1.0; //
                    if direction.length() > 0.0 {
                        direction = direction.normalize();
                    }

                    // Keep applying the velocity to the paddle
                    linear_velocity.x = direction.x * PADDLE_SPEED;
                    linear_velocity.y = 0.0; // No vertical movement for the paddle
                }
            }
            // The button is no longer being pressed or hovered (button is released)
            Interaction::None => {
                // When the button is no longer interacted with, stop the paddle movement
                if let Ok(mut linear_velocity) = velocity_query.get_single_mut() {
                    linear_velocity.x = 0.0;
                    linear_velocity.y = 0.0;
                }
            }
            _ => (),
        }
    }

    // Clamping the paddle's position based on the window size (avoiding movement out of bounds)
    if let Ok(mut transform) = transform_query.get_single_mut() {
        let window_width = window.width();
        let min_x = -window_width / 2.0 + PADDLE_WIDTH / 2.0;
        let max_x = window_width / 2.0 - PADDLE_WIDTH / 2.0;
        transform.translation.x = transform.translation.x.clamp(min_x, max_x);
    }
}
