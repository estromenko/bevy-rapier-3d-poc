use crate::AppState;
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
        let mut any_direction = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            any_direction += transform.forward();
        }
        if keys.pressed(KeyCode::S) {
            any_direction += transform.back();
        }
        if keys.pressed(KeyCode::A) {
            any_direction += transform.left();
        }
        if keys.pressed(KeyCode::D) {
            any_direction += transform.right();
        }

        if any_direction != Vec3::ZERO {
            let mut direction_without_y = any_direction.clone();
            direction_without_y.y = 0.;
            velocity.linvel =
                direction_without_y.normalize() * Vec3::new(PLAYER_SPEED, 0., PLAYER_SPEED);
        }
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
        GravityScale(0.),
        Collider::cuboid(0.2, 1., 0.2),
        Velocity::zero(),
        Damping {
            linear_damping: PLAYER_SPEED * 4.,
            ..default()
        },
        Camera3dBundle {
            transform: Transform {
                translation: Vec3::new(0., 2., 0.),
                scale: Vec3::new(1., 1., 0.3),
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Walls"),
        GameObject,
        SceneBundle {
            scene: asset_server.load("ROOM.glb#Scene0"),
            ..default()
        },
    ));

    commands.spawn((
        GameObject,
        PointLightBundle {
            transform: Transform::from_xyz(0.0, 4.0, 0.0),
            point_light: PointLight {
                intensity: 1600.0,
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
