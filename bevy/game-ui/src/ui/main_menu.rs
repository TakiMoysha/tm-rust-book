use bevy::prelude::*;
use bevy::ui::{widget::Button, Node, Val};

use crate::states::AppState;

#[derive(Component)]
pub struct MainMenuMarker;

pub fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.15)),
            MainMenuMarker,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("MY GAME"),
                TextFont {
                    font_size: 80.,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.8, 0.6)),
            ));

            parent.spawn(Node {
                height: Val::Px(60.),
                ..default()
            });

            parent
                .spawn((
                    Button,
                    PlayButton,
                    Node {
                        width: Val::Px(300.),
                        height: Val::Px(60.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(10.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("PLAY"),
                        TextFont {
                            font_size: 32.,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            parent
                .spawn((
                    Button,
                    OptionsButton,
                    Node {
                        width: Val::Px(300.),
                        height: Val::Px(60.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(10.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("OPTIONS"),
                        TextFont {
                            font_size: 32.,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            parent
                .spawn((
                    Button,
                    QuitButton,
                    Node {
                        width: Val::Px(300.),
                        height: Val::Px(60.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(10.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("QUIT"),
                        TextFont {
                            font_size: 32.,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct OptionsButton;

#[derive(Component)]
pub struct QuitButton;

pub fn play_button_pressed(
    query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::Playing);
        }
    }
}

pub fn options_button_pressed(
    query: Query<&Interaction, (Changed<Interaction>, With<OptionsButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::Options);
        }
    }
}

pub fn quit_button_pressed(query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            std::process::exit(0);
        }
    }
}

pub fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenuMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
