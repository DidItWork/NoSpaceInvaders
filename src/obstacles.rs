use std::ops::Range;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use std::f32::consts::PI;
use crate::{
    asset_loader::SceneAssets,
    movement::MovingObjectBundle,
};

const ASTEROID_VELOCITY: f32 = 5.0;
const ASTEROID_ACCELERATION: f32 = 1.0;
// const SPAWN_RANGE_X: Range<f32> = 0.0..25.0;
// const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;
const SPAWN_ANGLE: Range<f32> = 0.0..2.0*PI;
const SPAWN_DIST: f32 = 90.0;
const SPAWN_TIME_SECONDS: f32 = 1.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer : Timer,
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer {
            timer: Timer::from_seconds(SPAWN_TIME_SECONDS, TimerMode::Repeating),
        }).add_systems(Update, spawn_asteroid);
    }
}

fn spawn_asteroid(mut commands: Commands, mut spawn_timer: ResMut<SpawnTimer>, time: Res<Time>, scene_assets: Res<SceneAssets>) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let angle = rng.gen_range(SPAWN_ANGLE);

    let translation = Vec3::new(
        angle.cos(),
        0.,
        angle.sin(),
    );

    let mut random_unit_vector = 
        || (-translation.clone() + Vec3::new(rng.gen_range(-0.5..0.5), 0., rng.gen_range(-0.5..0.5))).normalize_or_zero();

    let velocity = random_unit_vector() * ASTEROID_VELOCITY;
    let acceleration = random_unit_vector() * ASTEROID_ACCELERATION;

    commands.spawn(RigidBody::Dynamic)
    .insert(Collider::ball(2.0))
    .insert(Restitution::coefficient(0.7))
    .insert(Velocity {
        linvel: velocity,
        angvel: Vec3::ZERO,
    })
    .insert(GravityScale(0.0))
    .insert((SceneBundle {
        scene: scene_assets.asteroid.clone(),
        transform: Transform::from_translation(translation * SPAWN_DIST),
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
    ,Asteroid));
}

// fn despawn_asteroid(mut commands: Commands, mut Query<(&Asteroid, &Window)>)