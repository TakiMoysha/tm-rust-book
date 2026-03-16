use bevy::prelude::*;

use playground::plugins;

fn main() {
    let _matches = clap::command!()
        .arg(clap::arg!(--test "run simple test_scena").action(clap::ArgAction::SetTrue))
        .get_matches();

    App::new()
        .add_plugins((
            DefaultPlugins,
            plugins::scenes::DemoScena3dPlugin,
            plugins::cameras::BootstrapCamera3dPlugin,
        ))
        // .add_systems(Startup,)
        // .add_systems(Update, (rotate_object, render_materials))
        .run();
}
