use bevy::prelude::*;
use bevy_egui::{EguiContextSettings, EguiPlugin, EguiPrimaryContextPass, egui};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .init_resource::<UiEditorState>()
            .add_systems(Startup, setup_ui_state_system)
            .add_systems(
                EguiPrimaryContextPass,
                (ui_editor_system, update_ui_scale_factor_system),
            );
    }
}
// ======================================================================

fn setup_ui_state_system() {}
// ======================================================================
#[derive(Default, Resource)]
struct UiEditorState {
    egui_texture_handle: Option<egui::TextureHandle>,
    is_window_open: bool,
}

fn ui_editor_system(mut ui_state: ResMut<UiEditorState>) {
    ui_state.is_window_open = true;
}

fn update_ui_scale_factor_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut toggle_scale_factor: Local<Option<bool>>,
    egui_context: Single<(&mut EguiContextSettings, &Camera)>,
) {
    let (mut egui_settings, camera) = egui_context.into_inner();
    if keyboard_input.just_pressed(KeyCode::Slash) || toggle_scale_factor.is_none() {
        *toggle_scale_factor = Some(!toggle_scale_factor.unwrap_or(true));

        let scale_factor = if toggle_scale_factor.unwrap() {
            1.
        } else {
            1. / camera.target_scaling_factor().unwrap_or(1.)
        };

        // egui_settings.0.scale = scale_factor;
    }
}
