use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::AppState;

const PLAYER_SPEED: f32 = 10.;
const CAMERA_ROTATION_SPEED: f32 = 0.01;

#[derive(Component, Reflect)]
pub struct Player;

pub struct PlayerPlugin;

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

pub fn spawn_player(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("Player"),
            Player,
            RigidBody::Dynamic,
            Collider::cuboid(1., 4., 1.),
            Velocity::zero(),
            Ccd::enabled(),
            GravityScale(PLAYER_SPEED * 4.),
            LockedAxes::ROTATION_LOCKED,
            Camera3dBundle {
                transform: Transform {
                    translation: Vec3::new(0., 20., 0.),
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>().add_systems(
            Update,
            (
                handle_movement.run_if(in_state(AppState::Game)),
                handle_mouse_motions.run_if(in_state(AppState::Game)),
            ),
        );
    }
}
