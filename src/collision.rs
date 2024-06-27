use bevy::prelude::*;
// use bevy_rapier3d::{prelude::*, pipeline::CollisionEvent};
use bevy_xpbd_3d::prelude::*;
use crate::{
    spaceship::{Score, Spaceship},
    health::Health,
    states::GameState,
    obstacles::Asteroid
};

const HITPOINTS: f32 = 5.0;
const SPACESHIP_MULTIPLIER: f32 = 1.0;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_for_collisions.run_if(in_state(GameState::InGame)));
    }
}

fn check_for_collisions(
    mut commands: Commands,
    spaceship: Query<Entity, With<Spaceship>>,
    asteroids: Query<Entity, With<Asteroid>>,
    mut healthy_objects: Query<&mut Health>,
    mut collision_events: EventReader<CollisionStarted>,
) {
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        // println!("Received collision event: {:?}", collision);
        if !asteroids.get(*entity1).is_ok() || !asteroids.get(*entity2).is_ok() {
            for &e in [entity1, entity2].iter() {
                if let Ok(mut health) = healthy_objects.get_mut(*e) {
                    // let mut spaceship_health = spaceship_health.single_mut();
                    // match spaceship_health.width {
                    //     Val::Percent(x) => spaceship_health.width = Val::Percent(f32::max(x-HITPOINTS, 0.0)),
                    //     _ => {}
                    // }
                    health.value -= HITPOINTS;
                } else {
                    commands.entity(*e).despawn_recursive();
                }
            }
        }  
    }
}