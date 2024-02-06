use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

use crate::{config::Config, AppState};

#[derive(Component, Reflect)]
pub struct Player;

pub struct PlayerPlugin;

fn handle_movement(
    mut player_query: Query<(&mut Velocity, &Children), With<Player>>,
    camera_query: Query<&Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    config: Res<Config>,
) {
    for (mut velocity, children) in &mut player_query {
        let camera = children.get(0).unwrap();
        let transform = camera_query.get(*camera).unwrap();

        let mut rotation = Vec3::ZERO;

        if keys.pressed(KeyCode::W) {
            rotation += transform.forward();
        } else if keys.pressed(KeyCode::S) {
            rotation += transform.back();
        }
        if keys.pressed(KeyCode::A) {
            rotation += transform.left();
        } else if keys.pressed(KeyCode::D) {
            rotation += transform.right();
        }

        if rotation != Vec3::ZERO {
            velocity.linvel =
                rotation * Vec3::new(config.movement_speed, 0., config.movement_speed);
        }
    }
}

fn handle_mouse_motions(
    mut mouse_motion_event: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    config: Res<Config>,
) {
    for event in mouse_motion_event.read() {
        for mut transform in &mut query {
            transform.rotate_y(-event.delta.x * config.mouse_sensibility);

            let y_rotation = transform.forward().y;
            let in_top_vertial_range = y_rotation - event.delta.y * config.mouse_sensibility < 0.8;
            let in_bottom_vertial_range =
                y_rotation - event.delta.y * config.mouse_sensibility > -0.8;

            if in_top_vertial_range && in_bottom_vertial_range {
                transform.rotate_local_x(-event.delta.y * config.mouse_sensibility);
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
            LockedAxes::ROTATION_LOCKED,
            Damping {
                linear_damping: 10.,
                ..default()
            },
            Collider::cuboid(1., 4., 1.),
            Velocity::zero(),
            TransformBundle::from_transform(Transform::from_xyz(0., 6., 0.)),
        ))
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(0., 4., 0.),
                ..default()
            });
        })
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
