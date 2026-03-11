use bevy::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    let app = App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .run();
}
