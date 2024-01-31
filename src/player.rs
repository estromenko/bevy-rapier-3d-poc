use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::AppState;

const PLAYER_SPEED: f32 = 1.;
const CAMERA_ROTATION_SPEED: f32 = 0.001;

#[derive(Component, Reflect)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

pub struct PlayerPlugin;

fn handle_movement(
    mut query: Query<(&Transform, &mut KinematicCharacterController), With<Player>>,
    keys: Res<Input<KeyCode>>,
) {
    for (transform, mut controller) in &mut query {
        let mut changed_velocity = controller.translation.unwrap_or(Vec3::ZERO);

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

        changed_velocity *= Vec3::new(PLAYER_SPEED, 1., PLAYER_SPEED);
        controller.translation = Some(changed_velocity.normalize());
    }
}

fn handle_mouse_motions(
    mut mouse_motion_event: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<PlayerCamera>>,
) {
    for event in mouse_motion_event.read() {
        for mut transform in &mut query {
            transform.rotate_y(-event.delta.x * CAMERA_ROTATION_SPEED);

            let y_rotation = transform.forward().y;
            let in_top_vertial_range = y_rotation - event.delta.y * CAMERA_ROTATION_SPEED < 0.8;
            let in_bottom_vertial_range = y_rotation - event.delta.y * CAMERA_ROTATION_SPEED > -0.8;

            if in_top_vertial_range && in_bottom_vertial_range {
                transform.rotate_local_x(-event.delta.y * CAMERA_ROTATION_SPEED);
            }
        }
    }
}

fn handle_manual_gravity(
    mut query: Query<&mut KinematicCharacterController, With<Player>>,
    rapier_config: Res<RapierConfiguration>,
) {
    for mut controller in &mut query {
        let mut translation = controller.translation.unwrap_or(Vec3::ZERO);
        translation.y = -rapier_config.gravity.y.abs();
        controller.translation = Some(translation);
    }
}

pub fn spawn_player(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("Player"),
            Player,
            RigidBody::KinematicVelocityBased,
            KinematicCharacterController {
                translation: Some(Vec3::new(0., 20., 0.)),
                ..default()
            },
            Collider::cuboid(1., 4., 1.),
            Velocity::zero(),
        ))
        .id()
}

pub fn spawn_player_camera(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            PlayerCamera,
            Camera3dBundle {
                transform: Transform {
                    translation: Vec3::new(0., 10., 0.),
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
                handle_manual_gravity.run_if(in_state(AppState::Game)),
            ),
        );
    }
}
