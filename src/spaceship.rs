use crate::{
    asset_loader::SceneAssets,
    movement::{Acceleration, Movement, MovementBundle, Velocity},
};
use bevy::prelude::*;

const INITIAL_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const INITIAL_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const SPACESHIP_MOVEMENT_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;

#[derive(Bundle)]
pub struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

#[derive(Component)]
pub struct Spaceship;

// ================================
// ======= SpaceshipPlugin ========
// ================================
pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
        app.add_systems(Update, spaceship_movement_control);
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
fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
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
            velocity: Default::default(),
            acceleration: Acceleration::new(Vec3::ZERO),
            mark: Movement,
        },
        Spaceship,
    ));
}

fn spaceship_movement_control(
    mut query: Query<(&mut Transform, &mut Velocity), (With<Movement>, With<Spaceship>)>,
    button_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    // ensure the player still alive
    if let Ok((mut transform, mut velocity)) = query.get_single_mut() {
        let rotation = 0.0;
        let roll = 0.0;
        let mut movement = 0.0;

        if button_input.pressed(KeyCode::KeyW) {
            movement = SPACESHIP_MOVEMENT_SPEED;
        } else if button_input.pressed(KeyCode::KeyS) {
            movement = -SPACESHIP_MOVEMENT_SPEED;
        } else {
            movement = 0.0;
        }

        velocity.value = -transform.forward() * movement;
    }
}
