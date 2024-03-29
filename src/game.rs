use crate::{player::spawn_player, AppState};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;

const LIGHT_COLOR: Color = Color::rgba(0.5, 0.5, 0.17, 1.);

#[derive(Component, Reflect)]
pub struct GameObject;

pub struct GamePlugin;

fn spawn_game_objects(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Room"),
        GameObject,
        AsyncSceneCollider::default(),
        SceneBundle {
            scene: asset_server.load("room.glb#Scene0"),
            transform: Transform::default(),
            ..default()
        },
    ));

    commands.spawn((
        GameObject,
        PointLightBundle {
            transform: Transform::from_xyz(0.0, 9.0, 0.0),
            point_light: PointLight {
                intensity: 4000.0,
                color: LIGHT_COLOR,
                shadows_enabled: true,
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Squirrel"),
        RigidBody::Dynamic,
        GameObject,
        Collider::capsule(Vec3::ZERO, Vec3::new(0., 1., 0.), 1.),
        Restitution::coefficient(2.),
        SceneBundle {
            scene: asset_server.load("squirrel.glb#Scene0"),
            transform: Transform::from_xyz(0., 5., 0.),
            ..default()
        },
    ));

    let entity = spawn_player(&mut commands);
    commands.entity(entity).insert(GameObject);

    commands.insert_resource(RapierConfiguration::default());
}

fn despawn_game_objects(
    mut commands: Commands,
    query: Query<Entity, With<GameObject>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }

    let mut primary_window = window_query.single_mut();
    primary_window.cursor.visible = true;
}

fn hide_cursor(mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = window_query.single_mut();
    primary_window.cursor.visible = false;
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameObject>()
            .add_systems(OnEnter(AppState::MainMenu), despawn_game_objects)
            .add_systems(OnExit(AppState::MainMenu), spawn_game_objects)
            .add_systems(OnEnter(AppState::Game), hide_cursor);
    }
}
