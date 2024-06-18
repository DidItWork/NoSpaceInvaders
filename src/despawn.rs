use bevy::prelude::*;
use crate::obstacles::Asteroid;

const DESPAWN_DISTANCE:f32 = 100.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_entities);
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform), With<Asteroid>>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        // println!("{}", distance);
        if distance > DESPAWN_DISTANCE {
            // println!("Despawning!");
            commands.entity(entity).despawn_recursive();
        }
    }
}