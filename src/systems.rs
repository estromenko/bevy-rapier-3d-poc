use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::AppState;

pub struct SystemsPlugin;

pub fn transition_to_main_menu_state(
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: ResMut<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();

        match app_state.get() {
            &AppState::Game => {
                app_state_next_state.set(AppState::MainMenu);
            }
            &AppState::MainMenu => {
                app_state_next_state.set(AppState::Game);
            }
        }

        if primary_window.cursor.grab_mode == CursorGrabMode::Locked {
            primary_window.cursor.grab_mode = CursorGrabMode::None;
            primary_window.cursor.visible = true;
        } else {
            primary_window.cursor.grab_mode = CursorGrabMode::Locked;
            primary_window.cursor.visible = false;
        }
    }
}

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, transition_to_main_menu_state);
    }
}
