use crate::movement::Velocity;
use bevy::prelude::*;

// ================================
// ======= SpaceshipPlugin ========
// ================================
pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

pub fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            ..Default::default()
        },
        Velocity::default(),
    ));
}
