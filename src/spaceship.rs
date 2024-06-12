use bevy::prelude::*;

use crate::movement::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 0.0);

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceships);
    }
}

//defers tasks to the commands to queue and run
fn spawn_spaceships(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpaceshipBundle {
        velocity: Velocity {
            value: STARTING_VELOCITY,
        },
        model: SceneBundle {
            scene: asset_server.load(""),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}