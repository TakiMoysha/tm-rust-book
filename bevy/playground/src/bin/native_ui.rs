use bevy::prelude::*;

mod native_ui {
    use bevy::color::palettes::tailwind;
    use bevy::color::palettes::tailwind::CYAN_400;
    use bevy::color::palettes::tailwind::EMERALD_500;
    use bevy::color::palettes::tailwind::PURPLE_100;
    use bevy::color::palettes::tailwind::PURPLE_700;
    use bevy::color::palettes::tailwind::YELLOW_400;
    use bevy::color::palettes::tailwind::YELLOW_900;
    use bevy::color::palettes::tailwind::ZINC_100;

    use bevy::ui::RelativeCursorPosition;
    use bevy::{
        app::{Plugin, Startup, Update},
        ecs::{children, system::Commands},
        prelude::*,
        ui::{
            BackgroundColor, BackgroundGradient, ColorStop, Display, Gradient, JustifyContent,
            LinearGradient, Node, Val, percent, px,
        },
        utils::default,
    };

    mod components {
        use super::*;

        pub fn button() -> impl Bundle {
            (
                Button,
                Node {
                    border: UiRect::all(px(4.)),
                    border_radius: BorderRadius::all(px(4.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(EMERALD_500),
                BackgroundColor(Color::BLACK),
                children![(
                    Text::new("Button"),
                    TextColor(tailwind::CYAN_700.into()),
                    TextShadow::default()
                )],
            )
        }

        pub fn on_hover_button(hover: On<Pointer<Over>>, mut commands: Commands) {
            commands
                .entity(hover.entity)
                .insert(BackgroundColor(YELLOW_400.into()));
        }

        pub fn button_system(
            mut interactions: Query<
                (
                    &Interaction,
                    &mut BackgroundColor,
                    &mut BorderColor,
                    &Children,
                ),
                (Changed<Interaction>, With<Button>),
            >,
            mut texts: Query<&mut Text>,
        ) {
            for (interaction, mut color, mut border_color, children) in &mut interactions {
                if let Ok(mut text) = texts.get_mut(children[0]) {
                    match *interaction {
                        Interaction::Pressed => {
                            text.0 = "Pressed".to_string();
                            *color = BackgroundColor(tailwind::ROSE_700.into());
                            *border_color = BorderColor::all(EMERALD_500);
                        }
                        Interaction::Hovered => {
                            text.0 = "Hovered".to_string();
                            *color = BackgroundColor(tailwind::ROSE_950.into());
                            *border_color = BorderColor::all(EMERALD_500);
                        }

                        Interaction::None => {
                            text.0 = "Button".to_string();
                            *color = BackgroundColor(Color::BLACK);
                            *border_color = BorderColor::all(EMERALD_500);
                        }
                    }
                }
            }
        }
    }

    const CELL_SIZE: f32 = 100.;
    const GAP: f32 = 40.;

    fn setup_ui(mut commands: Commands) {
        let column_container = Node {
            width: Val::Auto,
            height: Val::Auto,
            display: Display::Flex,
            ..default()
        };

        let row_container = Node {
            width: Val::Percent(40.0),
            height: percent(60.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Start,
            row_gap: px(GAP),
            column_gap: px(GAP),
            ..default()
        };

        let bg_1 = BackgroundGradient::from(LinearGradient::new(
            45.,
            vec![
                ColorStop::new(ZINC_100, px(5.)),
                ColorStop::new(PURPLE_700, px(5.)),
                ColorStop::new(YELLOW_400, percent(60.)),
                ColorStop::auto(CYAN_400),
            ],
        ));
        let bg_2 = BackgroundGradient::from(ConicGradient::new(
            UiPosition::center(percent(10), percent(10)),
            vec![
                AngularColorStop::new(CYAN_400, 50.),
                AngularColorStop::new(YELLOW_900, 120.),
                AngularColorStop::auto(tailwind::INDIGO_900),
            ],
        ));

        let card_bundle = (
            Node {
                width: Val::Auto,
                height: Val::Auto,
                ..default()
            },
            BackgroundColor(PURPLE_100.into()),
        );

        commands.spawn(Camera2d);

        commands
            .spawn((row_container, bg_1))
            .with_child(card_bundle)
            .with_children(|commands| {
                for i in 0..6 {
                    let position =
                        Vec2::new(i as f32 * (CELL_SIZE + GAP), i as f32 * (CELL_SIZE + GAP));
                    commands.spawn((
                        Text(format!("debug_value:\n{position:#?}")),
                        TextFont::from_font_size(14.),
                    ));
                }
            })
            .with_child((
                Text("debug_value:\n".to_string()),
                UiTransform {
                    translation: Val2::px(22., 10.),
                    rotation: Rot2::turn_fraction(0.12),
                    ..default()
                },
            ));
        commands
            .spawn((column_container.clone(), bg_2))
            .with_children(|parent| {
                parent
                    .spawn(components::button())
                    .observe(components::on_hover_button);
            });
    }

    fn relative_cursor_pos(cursor_query: Query<&RelativeCursorPosition>) {
        let Ok(cursor) = cursor_query.single() else {
            return;
        };

        if let Some(cursor) = cursor.normalized {
            info!("cursor: {cursor:#?}");
        }
    }

    pub struct NativeUiPlugin;

    impl Plugin for NativeUiPlugin {
        fn build(&self, app: &mut bevy::app::App) {
            app.add_systems(Startup, setup_ui)
                .add_systems(Update, components::button_system)
                .add_systems(Update, relative_cursor_pos)
                // .add_systems(Update, (mouse_update).chain())
            ;
        }
    }
}

fn main() {
    let app = App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(native_ui::NativeUiPlugin)
        .run();
}
