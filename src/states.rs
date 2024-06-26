use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::{
    spaceship::Spaceship,
    health::Health,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

#[derive(Component, Debug)]
pub struct PauseScreen;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(Update, (pause_system, game_over));
    }
}

fn pause_system(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    pausescreen: Query<Entity, With<PauseScreen>>,
    mut time: ResMut<Time<Physics>>,
    asset_server: Res<AssetServer>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => {
                next_state.set(GameState::Paused);
                time.pause();
                pause_screen(commands, asset_server);
            },
            GameState::Paused => {
                for entity in pausescreen.iter() {
                    commands.entity(entity).despawn_recursive();
                }
                next_state.set(GameState::InGame);
                time.unpause();
            },
            _ => {},
        }
    }
}

fn game_over(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    spaceship_health: Query<&Health, With<Spaceship>>,
) {
    match spaceship_health.get_single() {
        Ok(x) => {
            if *state.get() == GameState::InGame && x.value==0.0{
                next_state.set(GameState::GameOver)
            }
        },
        Err(..) => {},
    }    
}

fn pause_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((NodeBundle{
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::rgba(1.0, 1.0, 1.0, 0.1).into(),
        ..default()
    }, PauseScreen)).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
        "Paused",
        TextStyle {
            font: asset_server.load("fonts/FiraSansBold.ttf"),
            font_size: 30.0,
            color: Color::BLACK,
            ..default()
        }));
    });
}