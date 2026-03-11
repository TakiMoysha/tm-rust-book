pub mod scene {
    use bevy::prelude::*;

    use crate::states::AppState;

    pub fn setup_scene(mut commands: Commands) {
        commands.spawn(Camera2d);

        commands.spawn((
            Sprite {
                color: Color::srgb(0.8, 0.2, 0.2),
                custom_size: Some(Vec2::new(100., 100.)),
                ..default()
            },
            Transform::from_xyz(0., 0., 0.),
        ));
    }

    pub fn cleanup_scene(mut commands: Commands, query: Query<Entity, With<Camera2d>>) {
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
    }

    pub fn esc_to_menu(
        keys: Res<ButtonInput<KeyCode>>,
        mut next_state: ResMut<NextState<AppState>>,
    ) {
        if keys.just_pressed(KeyCode::Escape) {
            next_state.set(AppState::MainMenu);
        }
    }
}
