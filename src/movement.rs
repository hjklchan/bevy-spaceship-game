use bevy::prelude::*;

// =================================
// ======== MovementPlugin =========
// =================================
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {}
}


#[derive(Component, Debug, Default)]
pub struct Velocity {
    value: Vec3,
}
