use bevy::prelude::*;
use bevy::ui::{widget::Button, Node, Val};

use crate::states::AppState;

#[derive(Component)]
pub struct OptionsMarker;

#[derive(Component)]
pub struct BackButton;

pub fn setup_options(mut commands: Commands) {
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
            OptionsMarker,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("OPTIONS"),
                TextFont {
                    font_size: 60.,
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
                    BackButton,
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
                        Text::new("BACK"),
                        TextFont {
                            font_size: 32.,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn back_button_pressed(
    query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::MainMenu);
        }
    }
}

pub fn cleanup_options(mut commands: Commands, query: Query<Entity, With<OptionsMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
