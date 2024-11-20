use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

/// # spawn a main camera
/// 
/// Components:
/// - MainCamera#custom
/// - Camera3dBundle#bevy
fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera3dBundle {
            transform: Default::default(),
            ..Default::default()
        },
    ));
}