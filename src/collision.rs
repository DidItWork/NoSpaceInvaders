use bevy::prelude::*;
// use bevy_rapier3d::{prelude::*, pipeline::CollisionEvent};
use bevy_xpbd_3d::prelude::*;
use crate::{
    spaceship::Spaceship,
    hud::SpaceshipHealth,
    states::GameState,
    obstacles::Asteroid
};

const HITPOINTS: f32 = 5.0;

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
    mut spaceship_health: Query<&mut Style, With<SpaceshipHealth>>,
    mut collision_events: EventReader<CollisionStarted>,
) {
    for CollisionStarted(entity1, entity2) in collision_events.read() {
        // println!("Received collision event: {:?}", collision);
        if !asteroids.get(*entity1).is_ok() || !asteroids.get(*entity2).is_ok() {
            for &e in [entity1, entity2].iter() {
                if spaceship.get(*e).is_ok() {
                    let mut spaceship_health = spaceship_health.single_mut();
                    match spaceship_health.width {
                        Val::Percent(x) => spaceship_health.width = Val::Percent(f32::max(x-HITPOINTS, 0.0)),
                        _ => {}
                    }
                } else {
                    commands.entity(*e).despawn_recursive();
                }  
            }      
        }      
    }
}