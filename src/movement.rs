use std::ops::Deref;

use bevy::prelude::*;

// =================================
// ======== MovementPlugin =========
// =================================
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
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

impl Deref for Velocity {
    type Target = Vec3;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn update_position(mut query: Query<(&mut Transform, &Velocity), With<Movement>>, time: Res<Time>) {
    let delta = time.delta_seconds();

    for (mut transform, velocity) in query.iter_mut() {
        // `**velocity` can be dereferenced as a property of type Vec3 in Velocity structure,
        // so we can get normalize_or_zero via velocity.
        transform.translation += velocity.normalize_or_zero() * delta * 10.0;
    }
}
