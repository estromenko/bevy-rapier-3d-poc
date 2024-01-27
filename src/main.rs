use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

const LIGHT_COLOR: Color = Color::rgba(0.5, 0.5, 0.17, 1.);

#[derive(Component, Reflect)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform {
            translation: Vec3::new(0., 2., 0.),
            scale: Vec3::new(1., 1., 0.3),
            ..default()
        },
        ..default()
    });
    commands.spawn(Player);

    commands.spawn(SceneBundle {
        scene: asset_server.load("ROOM.glb#Scene0"),
        ..default()
    });

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

fn handle_movement() {}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_movement)
        .register_type::<Player>()
        .run();
}
