use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, pause_system);
    }
}

fn pause_system(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            _ => {},
        }
    }
}