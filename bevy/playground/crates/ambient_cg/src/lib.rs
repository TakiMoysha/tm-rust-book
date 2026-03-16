use bevy::image::ImageLoaderSettings;
use bevy::prelude::*;

pub struct AmbientCgPlugin;

impl Plugin for DemoScena3dPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            // .add_systems(Update, draw_cursor)
        ;
    }
}



struct AmbientCgMaterial;
