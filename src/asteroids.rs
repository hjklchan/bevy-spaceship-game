use std::ops::Range;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::Rng;

use crate::{asset_loader::SceneAssets, movement::{Acceleration, Movement, MovementBundle, Velocity}};

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_FIXED_Y: f32 = 10.0;
const MAX_QUANTITY: usize = 3;

pub struct AsteroidsPlugin;

impl Plugin for AsteroidsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpawnTimer>();
        app.add_event::<SpawnEvent>();
        app.add_systems(Update, spawn_asteroid);
        app.add_systems(Update, spawn_plan);
    }
}

#[derive(Component)]
pub struct Asteroid;

#[derive(Resource)]
pub struct SpawnTimer {
    timer: Timer,
}

#[derive(Event)]
pub struct SpawnEvent;

impl Default for SpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut spawn_event: EventWriter<SpawnEvent>,
) {
    let window = window.single();
    let _max_height = window.height() / 2.0;

    spawn_timer.timer.tick(time.delta());

    if spawn_timer.timer.finished() {
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(SPAWN_RANGE_X);
        let random_z = rng.gen_range(0.0..25.0);
        let translation = Vec3::new(random_x, 0.0, random_z);

        let mut random_unit_vector3 =
            || Vec3::new(rng.gen_range(-1.0..1.0), 0.0, rng.gen_range(-1.0..1.0));
        let random_velocity = random_unit_vector3() * 5.0;
        let random_acceleration = random_unit_vector3();

        commands.spawn((
            Asteroid,
            MovementBundle {
                velocity: Velocity::new(random_velocity),
                acceleration: Acceleration::new(random_acceleration),
                mark: Movement,
            },
            SceneBundle {
                scene: scene_assets.asteroid.clone(),
                transform: Transform {
                    translation,
                    ..Default::default()
                },
                ..Default::default()
            }
        ));

        // Send spawning event
        spawn_event.send(SpawnEvent);
    }
}

/// spawn_plan
/// 
/// TODO: Test whether the system will resume generating entities.
/// 
/// - The number of asteroid entities should be limited.
/// - Pause the [`SpawnTimer`] if number of asteroid entities gt than or eq to [`MAX_NUM_PRESENT`].
fn spawn_plan(mut spawn_timer: ResMut<SpawnTimer>, query: Query<(), With<Asteroid>>, mut spawn_event: EventReader<SpawnEvent>) {
    // If entity has been spawned,
    // Then query currently number of entities.
    for _ in spawn_event.read() {
        let count = query.iter().count();

        if count >= MAX_QUANTITY {
            spawn_timer.timer.pause();
        }
    }
}