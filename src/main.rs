mod camera;
mod debug;
mod movement;
mod spaceship;
mod asteroids;
mod asset_loader;

use crate::debug::DebugPlugin;
use crate::movement::MovementPlugin;
use crate::spaceship::SpaceshipPlugin;
use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidsPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // change background color
        .insert_resource(ClearColor(Color::srgb_u8(25, 25, 112)))
        // add light
        .insert_resource(AmbientLight {
            color: Color::srgb_u8(138, 43, 226),
            brightness: 0.75,
        })
        // Customized plugins for develops
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidsPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
