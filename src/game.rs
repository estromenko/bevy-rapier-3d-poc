use crate::{gltf_auto_colliders::GltfAsset, player::spawn_player, AppState};
use bevy::prelude::*;

const LIGHT_COLOR: Color = Color::rgba(0.5, 0.5, 0.17, 1.);

#[derive(Component, Reflect)]
pub struct GameObject;

pub struct GamePlugin;

fn spawn_game_objects(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GltfAsset(asset_server.load("room.glb")));

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

    let entity = spawn_player(&mut commands);
    commands.entity(entity).insert(GameObject);
}

fn despawn_game_objects(mut commands: Commands, query: Query<Entity, With<GameObject>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameObject>()
            .add_systems(OnEnter(AppState::Game), spawn_game_objects)
            .add_systems(OnExit(AppState::Game), despawn_game_objects);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Running,
    _Paused,
}
