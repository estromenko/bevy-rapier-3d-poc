use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier3d::prelude::*;

const PLAYER_SPEED: f32 = 10.;
const LIGHT_COLOR: Color = Color::rgba(0.5, 0.5, 0.17, 1.);
const CAMERA_ROTATION_SPEED: f32 = 0.01;

#[derive(Component, Reflect)]
pub struct Player;

pub struct GamePlugin;

fn handle_movement(mut query: Query<&mut Velocity, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut velocity = query.single_mut();

    if keys.pressed(KeyCode::W) {
        velocity.linvel.z = -PLAYER_SPEED;
    }
    if keys.pressed(KeyCode::S) {
        velocity.linvel.z = PLAYER_SPEED;
    }
    if keys.pressed(KeyCode::A) {
        velocity.linvel.x = -PLAYER_SPEED;
    }
    if keys.pressed(KeyCode::D) {
        velocity.linvel.x = PLAYER_SPEED;
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
            if event.delta.y < 0. {
                transform.rotate_local_x(CAMERA_ROTATION_SPEED);
            }
            if event.delta.y > 0. {
                transform.rotate_local_x(-CAMERA_ROTATION_SPEED);
            }
        }
    }
}

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>, keys: Res<Input<KeyCode>>) {
    let mut primary_window = q_windows.single_mut();

    if keys.just_pressed(KeyCode::Escape) {
        if primary_window.cursor.grab_mode == CursorGrabMode::Locked {
            primary_window.cursor.grab_mode = CursorGrabMode::None;
            primary_window.cursor.visible = true;
        } else {
            primary_window.cursor.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor.visible = false;
        }
    }
}

fn spawn_game_objects(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Name::new("Player"),
        Player,
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
        SceneBundle {
            scene: asset_server.load("ROOM.glb#Scene0"),
            ..default()
        },
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        point_light: PointLight {
            intensity: 1600.0,
            color: LIGHT_COLOR,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_systems(Startup, spawn_game_objects)
            .add_systems(Update, (handle_movement, cursor_grab, handle_mouse_motions));
    }
}
