use crate::{gltf_auto_colliders::GltfAsset, AppState};
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

const PLAYER_SPEED: f32 = 10.;
const LIGHT_COLOR: Color = Color::rgba(0.5, 0.5, 0.17, 1.);
const CAMERA_ROTATION_SPEED: f32 = 0.01;

#[derive(Component, Reflect)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct GameObject;

pub struct GamePlugin;

fn handle_movement(
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    for (mut velocity, transform) in &mut query {
        let mut changed_velocity = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            changed_velocity += transform.forward();
        }
        if keys.pressed(KeyCode::S) {
            changed_velocity += transform.back();
        }
        if keys.pressed(KeyCode::A) {
            changed_velocity += transform.left();
        }
        if keys.pressed(KeyCode::D) {
            changed_velocity += transform.right();
        }

        velocity.linvel = changed_velocity * Vec3::new(PLAYER_SPEED, 1., PLAYER_SPEED);
    }
}

fn handle_mouse_motions(
    mut mouse_motion_event: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for event in mouse_motion_event.read() {
        for mut transform in &mut query {
            if event.delta.x < 0. {
                transform.rotate_y(CAMERA_ROTATION_SPEED);
            }
            if event.delta.x > 0. {
                transform.rotate_y(-CAMERA_ROTATION_SPEED);
            }

            let y_rotation = transform.forward().y;

            if event.delta.y < 0. && y_rotation + CAMERA_ROTATION_SPEED < 1. {
                transform.rotate_local_x(CAMERA_ROTATION_SPEED);
            }
            if event.delta.y > 0. && y_rotation - CAMERA_ROTATION_SPEED > -1. {
                transform.rotate_local_x(-CAMERA_ROTATION_SPEED);
            }
        }
    }
}

fn spawn_game_objects(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Player"),
        Player,
        GameObject,
        RigidBody::Dynamic,
        Collider::cuboid(1., 4., 1.),
        Velocity::zero(),
        Ccd::enabled(),
        GravityScale(PLAYER_SPEED * 4.),
        Restitution::coefficient(0.),
        Damping {
            angular_damping: 1000000.,
            ..default()
        },
        Camera3dBundle {
            transform: Transform {
                translation: Vec3::new(0., 20., 0.),
                ..default()
            },
            ..default()
        },
    ));

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
}

fn despawn_game_objects(mut commands: Commands, query: Query<Entity, With<GameObject>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .register_type::<GameObject>()
            .add_systems(OnEnter(AppState::Game), spawn_game_objects)
            .add_systems(OnExit(AppState::Game), despawn_game_objects)
            .add_systems(
                Update,
                (handle_movement, handle_mouse_motions).run_if(in_state(AppState::Game)),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Running,
    _Paused,
}
