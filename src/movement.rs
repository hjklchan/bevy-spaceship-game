use bevy::prelude::*;

// =================================
// ======== MovementPlugin =========
// =================================
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_position, update_velocity));
    }
}

// Movement marking component
#[derive(Component)]
pub struct Movement;

#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub value: Vec3,
}

impl From<Vec3> for Velocity {
    fn from(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Component, Debug, Default)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub mark: Movement,
}

fn update_position(mut query: Query<(&mut Transform, &Velocity), With<Movement>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut transform, velocity) in query.iter_mut() {
        transform.translation += velocity.value.normalize_or_zero() * delta;
    }
}

fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * delta;
    }
}
