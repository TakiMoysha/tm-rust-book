// todo: 
    // - [] rotation the camera
    // - [] materials
    // - [] shaders
    // - [] lights
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

#[derive(Component, Debug)]
pub struct Ground;

// ################## PLUGINS
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(TextBundle::from_section(
        "Some 3D",
        TextStyle {
            font: asset_server.load("fonts/Vera.ttf"),
            font_size: 16.,
            color: Color::WHITE,
        },
    ));
    let sphere_handle = meshes.add(
        Mesh::try_from(shape::Icosphere {
            radius: 1.,
            subdivisions: 1,
            ..default()
        })
        .unwrap(),
    );
    commands.spawn((
        PbrBundle {
            mesh: sphere_handle,
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Ground,
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 128.0,
                subdivisions: 1,
            })),
            material: materials.add(Color::WHITE.into()),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ground,
    ));
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(5.0, 5.0, 0.0),
        point_light: PointLight {
            intensity: 0.0,
            range: 500.0f32,
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

// ##################
fn mouse_handler(
    mut commands: Commands,
    mut mousebtn_evr: EventReader<MouseButtonInput>,
    p_window: Query<&Window, With<PrimaryWindow>>,
) {
    for ev in mousebtn_evr.read() {
        // right and left mouse button
        match ev.state {
            ButtonState::Pressed => match p_window.single().cursor_position() {
                Some(pos) => {}
                None => {
                    error!("Mouse left button pressed, but no cursor position available");
                }
            },
            ButtonState::Released => debug!("Mouse left button released"),
        }
    }
}

// ################## SUPPORT
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (mouse_handler));
    }
}

