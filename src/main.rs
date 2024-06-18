mod obstacles;
mod debug;
mod movement;
mod spaceship;
mod camera;
mod asset_loader;
mod despawn;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use camera::CameraPlugin;
use obstacles::AsteroidPlugin;
use asset_loader::AssetLoaderPlugin;
use despawn::DespawnPlugin;


// fn spawn_camera(mut commands: Commands) {
//     commands.spawn(Camera2dBundle {
//         transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
//         ..default()
//     });
// }

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(SpaceshipPlugin)
        // .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(AsteroidPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(DespawnPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .run();
}
