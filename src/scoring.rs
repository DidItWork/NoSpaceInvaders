use bevy::prelude::*;
use crate::{
    states::GameState,
};

#[derive(Component, Debug)]
pub struct Score;

#[derive(Resource, Debug)]
pub struct ScoreTimer{
    timer: Timer,
}

pub struct ScorePlugin;

const SCORING_SECONDS: f32 = 10.0;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScoreTimer{
            timer: Timer::from_seconds(SCORING_SECONDS, TimerMode::Repeating),
    }).add_systems(Update, time_score.run_if(in_state(GameState::InGame)));
    }
}

pub fn time_score(
    time: Res<Time>,
    mut score_timer: ResMut<ScoreTimer>,
    mut score: Query<&mut Text, With<Score>>,
) {
    score_timer.timer.tick(time.delta());

    if let Ok(mut score) = score.get_single_mut() {
        if score_timer.timer.just_finished() {
            update_score(Some(1), &mut score);
        }
    }
}

pub fn update_score(
    update_val: Option<i32>,
    score: &mut Text,
) {
    let new_score = format!("{:0>8}", (score.sections[0].value.parse::<i32>().unwrap()+update_val.unwrap_or(1)).to_string());
    score.sections[0].value = new_score;
}