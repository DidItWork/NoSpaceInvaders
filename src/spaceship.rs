use bevy::{prelude::*, scene};
use bevy_rapier3d::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    // movement::{Velocity, Acceleration, MovingObjectBundle},
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATION_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const MISSILE_SPEED: f32 = 1.0;
const MISSILE_OFFSET: f32 = 7.5;
const FORCE_CONST: f32 = 10.0;
const TORQUE_CONST: f32 = 10.0;
const THROTTLE_LIMIT: f32 = 100.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceships)
            .add_systems(Update, (spaceship_movement_controls, spaceship_weapon_system));
    }
}

//defers tasks to the commands to queue and run
fn spawn_spaceships(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    // commands.spawn((
    //     MovingObjectBundle {
    //         velocity: Velocity {
    //             value: Vec3::ZERO,
    //         },
    //         acceleration: Acceleration {
    //             value: Vec3::ZERO,
    //         },
    //         model: SceneBundle {
    //             scene: scene_assets.spaceship.clone(),
    //             transform: Transform::from_translation(STARTING_TRANSLATION),
    //             ..default()
    //         },
    //     },
    //     Spaceship,
    // ));

    commands.spawn(RigidBody::KinematicVelocityBased)
    .insert(Collider::ball(1.5))
    .insert(Velocity{
        linvel: Vec3::ZERO,
        angvel: Vec3::ZERO,
    })
    .insert((SceneBundle {
        scene: scene_assets.spaceship.clone(),
        transform: Transform::from_translation(Vec3::ZERO),
        ..default()
    }
    // .insert((TransformBundle::from(Transform::from_translation(translation * SPAWN_DIST))
    // .insert((MovingObjectBundle {
    //     // velocity: Velocity::new(velocity),
    //     // acceleration: Acceleration::new(acceleration),
    //     model: SceneBundle {
    //         scene: scene_assets.asteroid.clone(),
    //         transform: Transform::from_translation(translation * SPAWN_DIST),
    //         ..default()
    //     },
    // },))
    ,Spaceship));
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Velocity, &Transform), With<Spaceship>>, //With is just for constraint no data from Spaceship is requried to be accessed
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    //Panics if there is more than one entity in query
    let (mut velocity, transform) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;
    // let mut throttle = -transform.forward().dot(ext_force.force);
    

    if keyboard_input.pressed(KeyCode::KeyW) {
        velocity.linvel = -transform.forward() * SPACESHIP_SPEED;
    } else {
        velocity.linvel = Vec3::ZERO;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        // ext_force.torque = Vec3::new(0.0, 1.0, 0.0) * TORQUE_CONST;
        velocity.angvel.y = SPACESHIP_ROTATION_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.angvel.y = -SPACESHIP_ROTATION_SPEED;
    } else {
        velocity.angvel.y = 0.0;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        // ext_force.torque += Vec3::new(0.0, 0.0, 1.0) * TORQUE_CONST;
        let roll_axis = -transform.forward();
        velocity.angvel.x = roll_axis.x * SPACESHIP_ROLL_SPEED;
        velocity.angvel.z = roll_axis.z * SPACESHIP_ROLL_SPEED;
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        // ext_force.torque += Vec3::new(0.0, 0.0, -1.0) * TORQUE_CONST;
        let roll_axis = transform.forward();
        velocity.angvel.x = roll_axis.x * SPACESHIP_ROLL_SPEED;
        velocity.angvel.z = roll_axis.z * SPACESHIP_ROLL_SPEED;
    } else{
        velocity.angvel.x = 0.0;
        velocity.angvel.z = 0.0;
    }

    // if throttle > THROTTLE_LIMIT{
    //     throttle = THROTTLE_LIMIT;
    // } else if throttle < -THROTTLE_LIMIT {
    //     throttle = -THROTTLE_LIMIT
    // }

    // println!("{}", throttle);

    // ext_force.force = -transform.forward() * throttle;

    // transform.rotate_y(rotation);

    // transform.rotate_local_z(roll);

    // acceleration.value = -transform.forward() * movement;

}

fn spaceship_weapon_system(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>
) {
//     let transform = query.single();

//     if keyboard_input.pressed(KeyCode::Space) {
//         //Fire Missiles
//         commands.spawn((
//             MovingObjectBundle{
//                 velocity:Velocity::new(-transform.forward() * MISSILE_SPEED),
//                 acceleration: Acceleration::new(Vec3::ZERO),
//                 model: SceneBundle {
//                     scene: scene_assets.missile.clone(),
//                     transform: Transform::from_translation(transform.translation - transform.forward() * MISSILE_OFFSET),
//                 ..default()
//                 }
//             },
//         SpaceshipMissile,
//     ));
    
//     }
}