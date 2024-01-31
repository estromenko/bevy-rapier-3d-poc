use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::RapierConfiguration;

use crate::AppState;

#[derive(Component, Reflect)]
pub struct PauseObject;

pub struct PausePlugin;

pub fn spawn_pause(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Pause overlay"),
            PauseObject,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    display: Display::Flex,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::BLACK.with_a(0.7).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Pause title"),
                PauseObject,
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "Pause",
                            TextStyle {
                                font_size: 48.,
                                ..default()
                            },
                        )],
                        ..default()
                    },
                    ..default()
                },
            ));
        });
}

pub fn despawn_pause(mut commands: Commands, query: Query<Entity, With<PauseObject>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_pause(
    keys: Res<Input<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    let mut primary_window = window_query.single_mut();

    if keys.just_pressed(KeyCode::Escape) {
        if state.get() == &AppState::Game {
            next_state.set(AppState::Pause);
            primary_window.cursor.visible = true;
            rapier_config.physics_pipeline_active = false;
        } else {
            next_state.set(AppState::Game);
            primary_window.cursor.visible = false;
            rapier_config.physics_pipeline_active = true;
        }
    }
}

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PauseObject>()
            .add_systems(
                Update,
                handle_pause.run_if(in_state(AppState::Game).or_else(in_state(AppState::Pause))),
            )
            .add_systems(OnEnter(AppState::Pause), spawn_pause)
            .add_systems(OnExit(AppState::Pause), despawn_pause);
    }
}
