use super::ui_components::*;
use crate::main_menu::MainMenuObject;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
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

    build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuObject>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
