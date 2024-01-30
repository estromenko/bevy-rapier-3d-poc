use bevy::prelude::*;

#[derive(Component)]
pub struct PauseObject;

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
