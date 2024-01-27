mod components;
mod styles;
mod systems;

use bevy::prelude::*;
use systems::layout::{despawn_main_menu, spawn_main_menu};

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            spawn_main_menu.run_if(in_state(AppState::MainMenu)),
        );
        app.add_systems(
            OnExit(AppState::MainMenu),
            despawn_main_menu.run_if(in_state(AppState::Game)),
        );
    }
}
