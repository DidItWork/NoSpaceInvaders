use bevy::{
    prelude::*
};

use crate::{
    health::Health,
    spaceship::{Score, Spaceship},
    states::GameState,
};

#[derive(Component, Debug)]
pub struct SpaceshipHealth;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (update_health, update_spaceship_health).run_if(in_state(GameState::InGame)));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(20.),
                margin: UiRect::top(Val::VMin(5.)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            parent.spawn(NodeBundle{
                style: Style {
                    width: Val::Percent(20.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            });
            parent.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(60.0),
                    height: Val::Percent(100.0),
                    border: UiRect::all(Val::Px(2.)),
                    ..default()
                },
                border_color: Color::ORANGE_RED.into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn((NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    background_color: Color::RED.into(),
                    ..default()
                }, SpaceshipHealth));
            });

            parent.spawn(NodeBundle{
                style: Style {
                    width: Val::Percent(20.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            }).with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                "Score:",
                TextStyle {
                    font: asset_server.load("fonts/FiraSansBold.ttf"),
                    font_size: 30.0,
                    ..default()
                }
            ));

            parent.spawn((TextBundle::from_section(
                "0",
                TextStyle {
                    font: asset_server.load("fonts/FiraSansBold.ttf"),
                    font_size: 30.0,
                    ..default()
                }
            ), Score));
            });
        });
    });
}

fn update_spaceship_health(
    mut spaceship_healthbar: Query<&mut Style, With<SpaceshipHealth>>,
    spaceship_health: Query<&Health, With<Spaceship>>,
) {

    let mut spaceship_healthbar = spaceship_healthbar.single_mut();
    let spaceship_health = spaceship_health.single();

    spaceship_healthbar.width = Val::Percent(f32::min(100.0,f32::max(spaceship_health.value, 0.0)));
    
}

fn update_health(
    mut healthbars: Query<(&mut Style, &Health)>,
) {
    for (mut healthbar, health) in healthbars.iter_mut() {
        healthbar.width = Val::Percent(f32::min(100.0,f32::max(health.value, 0.0)));
    }
}