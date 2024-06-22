use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, pipeline::CollisionEvent};
use crate::{
    spaceship::Spaceship,
    hud::SpaceshipHealth,
};

const HITPOINTS: f32 = 5.0;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_for_collisions);
    }
}

fn check_for_collisions(
    mut commands: Commands,
    spaceship: Query<Entity, With<Spaceship>>,
    mut spaceship_health: Query<&mut Style, With<SpaceshipHealth>>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for collision in collision_events.read() {
        // println!("Received collision event: {:?}", collision);
        match collision {
            CollisionEvent::Started(e1, e2, flag) => {
                for &e in [e1, e2].iter() {
                    if let Ok(x) = spaceship.get(*e) {
                        let mut spaceship_health = spaceship_health.single_mut();
                        match spaceship_health.width {
                            Val::Percent(x) => spaceship_health.width = Val::Percent(x-HITPOINTS),
                            _ => {}
                        }
                    } else {
                        commands.entity(*e).despawn_recursive();
                    }  
                }            
            }
            _ => {},
        }
    }
}