use avian2d::prelude::*;
use ball::BallPlugin;
use bevy::prelude::*;
use obstacle::ObstaclePlugin;
use paddle::PaddlePlugin;
use setup::SetupPlugin;
use ui::UIPlugin;

mod ball;
mod obstacle;
mod paddle;
mod setup;
mod ui;

// paddleBlue.png
fn main() {
    App::new()
        // .add_plugins(DefaultPlugins.set(WindowPlugin {
        //     primary_window: Some(Window {
        //         resolution: WindowResolution::new(720.0, 1280.0).with_scale_factor_override(1.0),
        //         resizable: false,
        //         ..Default::default()
        //     }),
        //     ..Default::default()
        // }))
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        // User Created Plugins
        .add_plugins((
            SetupPlugin,
            PaddlePlugin,
            BallPlugin,
            ObstaclePlugin,
            UIPlugin,
        ))
        .run();
}
