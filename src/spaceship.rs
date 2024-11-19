use crate::movement::{Movement, Velocity};
use bevy::prelude::*;

const INITIAL_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const INITIAL_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
pub struct SpaceshipBundle {
    model: SceneBundle,
}

// ================================
// ======= SpaceshipPlugin ========
// ================================
pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

pub fn spawn_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    // spawn a new spaceship
    //  - SceneBundle   #bevy
    //  - Velocity      #custom
    //  - Movement      #custom
    commands.spawn((
        SceneBundle {
            transform: Transform {
                translation: INITIAL_TRANSLATION,
                ..Default::default()
            },
            scene: asset_server.load(""),
            ..Default::default()
        },
        Velocity::from(INITIAL_VELOCITY), // with Velocity
        Movement, // with Movement mark
    ));
}
