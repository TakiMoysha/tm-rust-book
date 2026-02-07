use bevy::prelude::*;

// use crate::camera_controller::rts_camera_controller_plugin;
// pub mod camera_controller {
//     use bevy::{app::App, ecs::component::Component};
//
//     use crate::camera_controller::rts_camera_controller::RtsCameraControls;
//
//     // mod blender_camera_controller {}
//     // pub fn blender_camera_controller_plugin(app: &mut App) {}
//
//     #[derive(Component, Copy, Clone, Debug, Reflect)]
//     #[require(Camera3d)]
//     pub struct RtsCamera {
//         zoom: f32,
//     }
//
//     impl Default for RtsCamera {
//         fn default() -> Self {
//             Self { zoom: 0.0 }
//         }
//     }
//
//     mod rts_camera_controller {
//         use bevy::{
//             ecs::{component::Component, message::MessageReader, system::Query},
//             input::mouse::MouseWheel,
//             reflect::Reflect,
//         };
//
//         #[derive(Component, Debug, PartialEq, Clone, Reflect)]
//         pub struct RtsCameraControls {
//             pub enabled: bool,
//         }
//
//         impl Default for RtsCameraControls {
//             fn default() -> Self {
//                 Self { enabled: true }
//             }
//         }
//
//         pub fn zoom(mut mouse_wheel: MessageReader<MouseWheel>) {}
//     }
//
//     pub fn rts_camera_controller_plugin(app: &mut App) {
//         app.init_resource::<RtsCameraControls>()
//     }
// }

// ===========================================================
// setup
// ===========================================================

#[derive(Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20., 20.))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Ground,
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn draw_cursor(
    camera_query: Single<(&Camera, &GlobalTransform)>,
    ground: Single<&GlobalTransform, With<Ground>>,
    window: Single<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;

    if let Some(cursor_position) = window.cursor_position()
        // Calculate a ray pointing from the camera into the world based on the cursor's position.
        && let Ok(ray) = camera.viewport_to_world(camera_transform, cursor_position)
        // Calculate if and where the ray is hitting the ground plane.
        && let Some(point) = ray.plane_intersection_point(ground.translation(), InfinitePlane3d::new(ground.up()))
    {
        // Draw a circle just above the ground plane at that position.
        gizmos.circle(
            Isometry3d::new(
                point + ground.up() * 0.01,
                Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
            ),
            0.2,
            Color::WHITE,
        );
    }
}

// ===========================================================
// main
// ===========================================================

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(rts_camera_controller_plugin)
        // .add_systems(Startup, setup)
        // .add_systems(Update, draw_cursor)
        // .init_resource::<CameraState>()
        // .add_systems(Startup, (setup_scene, setup_lighting, setup_camera))
        // .add_systems(Update, (camera_controller, mouse_button_input))
        .run();
}
