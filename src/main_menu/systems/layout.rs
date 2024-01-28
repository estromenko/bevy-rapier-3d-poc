use crate::main_menu::components::{MainMenu, MainMenuObject};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_main_menu(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands.spawn((
        MainMenuObject,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            background_color: Color::PURPLE.into(),
            ..default()
        },
        MainMenu {},
    ));

    let primary_window = window_query.single_mut();

    commands.spawn((
        MainMenuObject,
        Camera2dBundle {
            transform: Transform::from_xyz(
                primary_window.width() / 2.0,
                primary_window.height() / 2.0,
                0.0,
            ),
            ..default()
        },
    ));
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuObject>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
