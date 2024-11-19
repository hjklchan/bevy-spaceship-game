use bevy::prelude::*;

// =================================
// ======== MovementPlugin =========
// =================================
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {}
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