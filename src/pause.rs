use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::prelude::RapierConfiguration;

use crate::AppState;

#[derive(Component, Reflect)]
pub struct PauseObject;

#[derive(Component, Reflect)]
struct QuitButton;

#[derive(Component, Reflect)]
struct ResumeButton;

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
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::BLACK.with_a(0.7).into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Pause title"),
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

            parent
                .spawn((
                    Name::new("Resume button"),
                    ResumeButton,
                    ButtonBundle {
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        style: Style {
                            width: Val::Px(400.),
                            height: Val::Px(100.),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Resume button text"),
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Resume",
                                    TextStyle {
                                        font_size: 64.,
                                        ..default()
                                    },
                                )],
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
            parent
                .spawn((
                    Name::new("Quit button"),
                    QuitButton,
                    ButtonBundle {
                        background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                        style: Style {
                            width: Val::Px(400.),
                            height: Val::Px(100.),
                            display: Display::Flex,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Name::new("Quit button text"),
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "Quit to menu",
                                    TextStyle {
                                        font_size: 64.,
                                        ..default()
                                    },
                                )],
                                ..default()
                            },
                            ..default()
                        },
                    ));
                });
        });
}

fn handle_quit_button_interaction(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::Hovered => {
                *background_color = Color::rgb(0., 0., 0.).into();
            }
            Interaction::Pressed => {
                next_app_state.set(AppState::MainMenu);
            }
            Interaction::None => {
                *background_color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

fn handle_resume_button_interaction(
    mut query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeButton>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    for (interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::Hovered => {
                *background_color = Color::rgb(0., 0., 0.).into();
            }
            Interaction::Pressed => {
                next_app_state.set(AppState::Game);
                rapier_config.physics_pipeline_active = true;
            }
            Interaction::None => {
                *background_color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
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
            rapier_config.physics_pipeline_active = true;
        }
    }
}

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PauseObject>()
            .add_systems(
                Update,
                (
                    handle_pause
                        .run_if(in_state(AppState::Game).or_else(in_state(AppState::Pause))),
                    handle_quit_button_interaction.run_if(in_state(AppState::Pause)),
                    handle_resume_button_interaction.run_if(in_state(AppState::Pause)),
                ),
            )
            .add_systems(OnEnter(AppState::Pause), spawn_pause)
            .add_systems(OnExit(AppState::Pause), despawn_pause);
    }
}
