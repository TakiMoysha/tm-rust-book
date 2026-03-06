use bevy::prelude::*;

mod game;
mod ui;

mod states {
    use bevy::prelude::States;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, States, Default)]
    pub enum AppState {
        LoadingScreen,
        #[default]
        MainMenu,
        Options,
        Playing,
        InGameMenu,
    }
}

use bevy::render::RenderPlugin;
use bevy::render::settings::WgpuSettings;
use game::scene;
use states::AppState;
use ui::main_menu;
use ui::options;

fn main() {
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "float".into(),
            resolution: (1280u32, 720u32).into(),
            ..default()
        }),
        ..default()
    };
    let render_plugin = RenderPlugin { ..default() };

    App::new()
        .add_plugins(DefaultPlugins.set(window_plugin).set(render_plugin))
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::MainMenu), main_menu::setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), main_menu::cleanup_main_menu)
        .add_systems(OnEnter(AppState::Options), options::setup_options)
        .add_systems(OnExit(AppState::Options), options::cleanup_options)
        .add_systems(OnEnter(AppState::Playing), scene::setup_scene)
        .add_systems(OnExit(AppState::Playing), scene::cleanup_scene)
        .add_systems(
        .add_systems(
            Update,
            (
                main_menu::play_button_pressed,
                main_menu::options_button_pressed,
                main_menu::quit_button_pressed,
                options::back_button_pressed,
                scene::esc_to_menu.run_if(in_state(AppState::Playing)),
            ),
        )
        .run();
}
