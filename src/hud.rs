use bevy::{
    prelude::*
};

pub struct HUDPlugin;

#[derive(Component, Debug)]
pub struct SpaceshipHealth;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands
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
        });
    });
}