use bevy::{
    prelude::*,
};
use bevy_xpbd_3d::prelude::*;

use crate::{
    asset_loader::SceneAssets,
    states::GameState,
    health::{Health, Healthbar, HealthbarVal},
    // movement::{Velocity, Acceleration, MovingObjectBundle},
};

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const SPACESHIP_SPEED: f32 = 15.0;
const SPACESHIP_ROTATION_SPEED: f32 = 1.0;
const SPACESHIP_ROLL_SPEED: f32 = 1.0;
const MISSILE_SPEED: f32 = 30.0;
const MISSILE_OFFSET: f32 = 7.5;
const FORCE_CONST: f32 = 10.0;
const TORQUE_CONST: f32 = 10.0;
const THROTTLE_LIMIT: f32 = 100.0;
const MISSILE_RATE: f32 = 5.0;

#[derive(Component, Debug)]
pub struct Spaceship;

#[derive(Component, Debug)]
pub struct SpaceshipMissile;

#[derive(Component, Debug)]
pub struct Cooldown(Timer);

#[derive(Component, Debug)]
pub struct Score;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceships)
            .add_systems(Update, (spaceship_cooldowns, spaceship_movement_controls, spaceship_weapon_system).run_if(in_state(GameState::InGame)));
    }
}

//defers tasks to the commands to queue and run
fn spawn_spaceships(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    commands.spawn(RigidBody::Kinematic)
    .insert(Collider::sphere(1.5))
    .insert(Sensor)
    // .insert(ActiveEvents::COLLISION_EVENTS)
    .insert(LinearVelocity(Vec3::ZERO))
    .insert(AngularVelocity(Vec3::ZERO))
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
    ,Spaceship))
    .insert(Health{
        value: 100.0,
        max: 100.0,
    })
    .with_children(|parent|{
        parent.spawn((PbrBundle {
        mesh: meshes.add(Cuboid::new(10., 0., 1.)),
        material: materials.add(StandardMaterial{
            base_color: Color::rgba(0.0, 0.0, 0.0, 0.5),
            ..default()
        }),
        // global_transform: GlobalTransform::from_translation(Vec3::new(0., 0., 10.)),
        transform: Transform::from_translation(Vec3::new(0., 2., 5.)),
        ..default()
        }, Healthbar)).with_children(|parent| {
            parent.spawn((PbrBundle {
                mesh: meshes.add(Cuboid::new(10.,0.,1.0)),
                material: materials.add(StandardMaterial{
                    base_color: Color::RED,
                    ..default()
                }),
                ..default()
            }, HealthbarVal));
        });

        // parent.spawn((NodeBundle{
        //     style: Style {
        //         height: Val::Px(100.),
        //         width: Val::Px(50.),
        //         ..default()
        //     },
        //     transform: Transform::from_translation(Vec3::ZERO),
        //     ..default()
        // },UiImage::new(scene_assets.healthbar.clone())));
    });
}

fn spaceship_movement_controls(
    mut query: Query<(&mut AngularVelocity, &mut LinearVelocity, &Transform), With<Spaceship>>, //With is just for constraint no data from Spaceship is requried to be accessed
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    //Panics if there is more than one entity in query
    let (mut angular_velocity, mut linear_velocity, transform) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;
    // let mut throttle = -transform.forward().dot(ext_force.force);

    if keyboard_input.pressed(KeyCode::KeyW) {
        let desired_vel = -transform.forward() * SPACESHIP_SPEED;
        linear_velocity.x = desired_vel.x;
        linear_velocity.y = desired_vel.y;
        linear_velocity.z = desired_vel.z;
    } else {
        linear_velocity.x = 0.0;
        linear_velocity.y = 0.0;
        linear_velocity.z = 0.0;
    }

    if keyboard_input.pressed(KeyCode::KeyA) {
        // ext_force.torque = Vec3::new(0.0, 1.0, 0.0) * TORQUE_CONST;
        angular_velocity.y = SPACESHIP_ROTATION_SPEED;
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        angular_velocity.y = -SPACESHIP_ROTATION_SPEED;
    } else {
        angular_velocity.y = 0.0;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        // ext_force.torque += Vec3::new(0.0, 0.0, 1.0) * TORQUE_CONST;
        let roll_axis = -transform.forward();
        angular_velocity.x = roll_axis.x * SPACESHIP_ROLL_SPEED;
        angular_velocity.z = roll_axis.z * SPACESHIP_ROLL_SPEED;
    } else if keyboard_input.pressed(KeyCode::ControlLeft) {
        // ext_force.torque += Vec3::new(0.0, 0.0, -1.0) * TORQUE_CONST;
        let roll_axis = transform.forward();
        angular_velocity.x = roll_axis.x * SPACESHIP_ROLL_SPEED;
        angular_velocity.z = roll_axis.z * SPACESHIP_ROLL_SPEED;
    } else{
        angular_velocity.x = 0.0;
        angular_velocity.z = 0.0;
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
    query: Query<(Entity, &Transform), With<Spaceship>>,
    cooldowns: Query<&Cooldown, With<Spaceship>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let (spaceship, transform) = query.single();

    if keyboard_input.pressed(KeyCode::Space) {
        //Fire Missiles

        if let Ok(cooldown) = cooldowns.get(spaceship){
            // println!("{}", cooldown.0.fraction() * 100.0);
            return;
        } else {
            commands.entity(spaceship).insert(
                Cooldown(
                    Timer::from_seconds(
                        1.0/MISSILE_RATE,
                        TimerMode::Once,
                    )
                )
            );
        }

        commands.spawn(RigidBody::Kinematic)
        .insert(Collider::sphere(0.1))
        .insert(Sensor)
        // .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(LinearVelocity(-transform.forward() * MISSILE_SPEED))
        .insert((
            SceneBundle {
                scene: scene_assets.missile.clone(),
                transform: Transform::from_translation(transform.translation - transform.forward() * MISSILE_OFFSET),
                ..default()
            },
        SpaceshipMissile));
    
    }
}

fn spaceship_cooldowns(
    mut commands: Commands,
    mut cooldowns: Query<(Entity, &mut Cooldown)>,
    time: Res<Time>
) {
    for (entity, mut cooldown) in &mut cooldowns {
        cooldown.0.tick(time.delta());

        if cooldown.0.finished() {
            commands.entity(entity).remove::<Cooldown>();
        }
    }
}