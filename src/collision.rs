use bevy::prelude::*;

#[derive(Event, Default)]
pub struct CollisionEvent;

#[derive(Component, Debug)]
pub struct Collider;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, spawn_spaceships);
    }
}

fn check_for_collisions(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform,), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {

}