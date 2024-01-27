use crate::main_menu::components::MainMenu;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_main_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands.spawn((
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

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            primary_window.width() / 2.0,
            primary_window.height() / 2.0,
            0.0,
        ),
        ..default()
    });
}

pub fn despawn_main_menu(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
    camera_query: Query<Entity, With<Camera2d>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn();
    }
    if let Ok(camera_entity) = camera_query.get_single() {
        commands.entity(camera_entity).despawn();
    }
}
