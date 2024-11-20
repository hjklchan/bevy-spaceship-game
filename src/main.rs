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
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        // add light
        .insert_resource(AmbientLight {
            color: Default::default(),
            brightness: 100.0,
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
