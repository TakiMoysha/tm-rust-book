pub mod draw_on_surface {
    use bevy::{
        core_pipeline::tonemapping::Tonemapping, gizmos, prelude::*, window::WindowResized,
    };

    fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands.spawn((
            Camera2dBundle {
                camera: Camera {
                    hdr: true, // HDR for bloom
                    ..default()
                },
                tonemapping: Tonemapping::TonyMcMapface, // for white balance
                ..default()
            },
            // BloomSettings::default(), // enable bloom for camera
        ));

        // Sprites
        commands.spawn(TextBundle::from_section(
            "Particles",
            TextStyle {
                font: asset_server.load("fonts/Vera.ttf"),
                font_size: 16.,
                color: Color::WHITE,
            },
        ));
    }

    fn update(mut gizmos: Gizmos, time: Res<Time>) {
        let sin = time.elapsed_seconds().sin() * 50.;
        gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED)
    }

    fn mouse_button_input(buttons: Res<Input<MouseButton>>) {
        if buttons.just_pressed(MouseButton::Left) {
            println!("left mouse button pressed: {:?}", buttons);
        }
    }

    fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
        for event in resize_reader.iter() {
            println!("window resized: {:?}", event);
        }
    }

    pub struct StaterdDefaultPlugins;
    impl Plugin for StaterdDefaultPlugins {
        fn build(&self, app: &mut App) {
            use bevy::input::common_conditions::*;

            app.add_systems(Startup, setup)
                .add_systems(Update, update)
                .add_systems(
                    Update,
                    (
                        mouse_button_input.run_if(input_pressed(MouseButton::Left)),
                        on_resize_system,
                    ),
                );
        }
    }
}

use bevy::{prelude::*, window::WindowResolution};

fn make_visible(mut window: Query<&mut Window>) {
    window.single_mut().visible = true;
}

fn main() {
    let app_primary_window = Window {
        title: "Hello World".to_string(),
        resolution: WindowResolution::new(460., 370.),
        present_mode: bevy::window::PresentMode::Fifo,
        visible: false,
        ..default()
    };
    let overwrite_defaul_plugin = WindowPlugin {
        primary_window: Some(app_primary_window),
        ..default()
    };
    App::new()
        .add_systems(Startup, make_visible)
        .add_plugins((
            DefaultPlugins.set(overwrite_defaul_plugin),
            draw_on_surface::StaterdDefaultPlugins,
        ))
        .run();
}
