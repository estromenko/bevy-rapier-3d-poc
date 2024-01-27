use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::game::{Lights, Player, Walls};
use crate::AppState;

pub struct SystemsPlugin;

pub fn transition_to_main_menu_state(
    mut commands: Commands,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    player_query: Query<Entity, With<Player>>,
    walls_query: Query<Entity, With<Walls>>,
    lights_query: Query<Entity, With<Lights>>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: ResMut<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        let mut primary_window = q_windows.single_mut();

        match app_state.get() {
            &AppState::Game => {
                app_state_next_state.set(AppState::MainMenu);

                if let Ok(player_entity) = player_query.get_single() {
                    commands.entity(player_entity).despawn();
                }
                if let Ok(walls_entity) = walls_query.get_single() {
                    commands.entity(walls_entity).despawn();
                }
                if let Ok(lights_entity) = lights_query.get_single() {
                    commands.entity(lights_entity).despawn();
                }
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
