use crate::{asset_loader::SceneAssets, movement::{Acceleration, Movement, MovementBundle, Velocity}};
use bevy::prelude::*;

const INITIAL_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const INITIAL_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
pub struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

// ================================
// ======= SpaceshipPlugin ========
// ================================
pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
    }
}

/// # spawn a new spaceship
/// 
/// *TODO: create a new bundle to encapsulate the following components*,
/// just like [`SpaceshipBundle`] structure.
///
/// Components:
/// - SceneBundle       #bevy
/// - MovementBundle    #custom
pub fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SceneBundle {
            transform: Transform {
                translation: INITIAL_TRANSLATION,
                ..Default::default()
            },
            scene: scene_assets.spaceship.clone(),
            ..Default::default()
        },
        MovementBundle {
            velocity: INITIAL_VELOCITY.into(),
            acceleration: Acceleration::new(Vec3::ZERO),
            mark: Movement,
        },
    ));
}
