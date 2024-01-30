mod game;
mod gltf_auto_colliders;
mod main_menu;
mod pause;
mod player;

use game::GamePlugin;
use gltf_auto_colliders::GltfAutoCollidersPlugin;
use main_menu::MainMenuPlugin;
use player::PlayerPlugin;

use bevy::{prelude::*, window::WindowMode};
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            }),
            #[cfg(debug_assertions)]
            WorldInspectorPlugin::new(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            #[cfg(debug_assertions)]
            RapierDebugRenderPlugin::default(),
            PlayerPlugin,
            GamePlugin,
            MainMenuPlugin,
            GltfAutoCollidersPlugin,
        ))
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    Pause,
}
