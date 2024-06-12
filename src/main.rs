mod debug;
mod movement;
mod spaceship;
mod camera;

use bevy::prelude::*;

use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;
use camera::CameraPlugin;


// fn spawn_camera(mut commands: Commands) {
//     commands.spawn(Camera2dBundle {
//         transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
//         ..default()
//     });
// }

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(SpaceshipPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}
