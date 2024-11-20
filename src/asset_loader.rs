use bevy::prelude::*;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>();
        app.add_systems(Startup, load_scene_assets);
    }
}

#[derive(Resource, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
}

fn load_scene_assets(asset_server: Res<AssetServer>, mut scene_assets: ResMut<SceneAssets>) {
    let asteroid = asset_server.load::<Scene>("Asteroid.glb#Scene0");
    let spaceship = asset_server.load::<Scene>("Spaceship.glb#Scene0");
    
    *scene_assets = SceneAssets {
        asteroid,
        spaceship,
    };
}
