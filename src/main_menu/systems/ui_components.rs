use crate::main_menu::MainMenuObject;
use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.35, 0.75, 0.35);

pub const BUTTON_STYLE: Style = {
    let mut style = Style::DEFAULT;
    style.justify_content = JustifyContent::Center;
    style.align_items = AlignItems::Center;
    style.height = Val::Px(100.0);
    style.width = Val::Px(250.0);
    style
};

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct QuitButton;

fn get_button_text_style(asset_server: &Res<AssetServer>) -> TextStyle {
    TextStyle {
        font: asset_server.load("fonts/Hack-Regular.ttf"),
        font_size: 64.0,
        color: Color::WHITE,
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let main_menu_container = commands
        .spawn((
            MainMenuObject,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    row_gap: Val::Px(50.0),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::PURPLE.into(),
                ..default()
            },
        ))
        .id();

    let title = commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "The Box",
                    get_button_text_style(&asset_server),
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        })
        .id();

    let play_button = commands
        .spawn((
            PlayButton,
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Play",
                        get_button_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        })
        .id();

    let quit_button = commands
        .spawn((
            QuitButton,
            ButtonBundle {
                style: BUTTON_STYLE,
                background_color: NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "Quit",
                        get_button_text_style(&asset_server),
                    )],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        })
        .id();

    commands
        .entity(main_menu_container)
        .push_children(&[title, play_button, quit_button]);
}
