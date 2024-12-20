use bevy::{core_pipeline::tonemapping::Tonemapping, gizmos, prelude::*, window::WindowResized};

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
        "Draw on surface",
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

pub struct CorePlugins;
impl Plugin for CorePlugins {
    fn build(&self, app: &mut App) {
        use bevy::input::common_conditions::*;

        app.add_systems(Startup, setup)
            .add_systems(Update, update)
            .add_systems(
                Update,
                (
                    mouse_button_input.run_if(input_pressed(MouseButton::Left)),
                ),
            );
    }
}
