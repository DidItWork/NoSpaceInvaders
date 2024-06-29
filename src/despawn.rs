use bevy::{prelude::*};
use crate::{
    obstacles::Asteroid,
    health::Health,
    states::GameState,
    scoring::{update_score, Score},
};

const DESPAWN_DISTANCE:f32 = 150.0;

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (despawn_far_away_entities, despawn_health).run_if(in_state(GameState::InGame)))
            .add_systems(Update, despawn_everything.run_if(in_state(GameState::GameOver)));
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform), With<Asteroid>>) {
    for (entity, transform) in query.iter() {
        let distance = transform.translation().distance(Vec3::ZERO);
        // println!("{}", distance);
        if distance > DESPAWN_DISTANCE {
            println!("Despawning!");
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_health(
    mut commands: Commands,
    health: Query<(Entity, &Health)>,
    asteroids: Query<&Asteroid>,
    mut score: Query<&mut Text, With<Score>>,
) {

    let scoring = score.get_single().is_ok();

    for (entity, hp) in health.iter() {
        if hp.value <= 0. {
            if asteroids.get(entity).is_ok() && scoring {
                update_score(Some(5), &mut score.get_single_mut().unwrap());
            }
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_everything(
    mut commands: Commands,
    entities: Query<Entity, With<Health>>,
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}