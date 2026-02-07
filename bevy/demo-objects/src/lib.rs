use bevy::prelude::*;
// use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};

// #[derive(Component)]
// pub struct MainCamera;

// #[derive(Resource, Default)]
// pub struct CameraState {
//     focus: Vec3,
//     radius: f32,
//     pitch: f32,
//     yaw: f32,
//     is_orbiting: bool,
//     is_panning: bool,
// }
//
// pub fn setup_scene(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // Ground plane
//     commands.spawn((
//         Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
//         MeshMaterial3d(materials.add(StandardMaterial {
//             base_color: Color::srgb(0.2, 0.5, 0.2),
//             ..default()
//         })),
//         Transform::from_xyz(0.0, 0.0, 0.0),
//     ));
//
//     // Grid lines for reference
//     let grid_material = materials.add(StandardMaterial {
//         base_color: Color::srgba(0.5, 0.5, 0.5, 0.3),
//         alpha_mode: AlphaMode::Blend,
//         ..default()
//     });
//
//     for i in -5..=5 {
//         let offset = i as f32 * 5.0;
//         // X lines
//         commands.spawn((
//             Mesh3d(meshes.add(Cuboid::new(50.0, 0.05, 0.05))),
//             MeshMaterial3d(grid_material.clone()),
//             Transform::from_xyz(0.0, 0.01, offset),
//         ));
//         // Z lines
//         commands.spawn((
//             Mesh3d(meshes.add(Cuboid::new(0.05, 0.05, 50.0))),
//             MeshMaterial3d(grid_material.clone()),
//             Transform::from_xyz(offset, 0.01, 0.0),
//         ));
//     }
//
//     // Spawn procedural objects
//     spawn_procedural_tree(
//         &mut commands,
//         &mut meshes,
//         &mut materials,
//         Vec3::new(-5.0, 0.0, -5.0),
//     );
//     spawn_procedural_rock(
//         &mut commands,
//         &mut meshes,
//         &mut materials,
//         Vec3::new(5.0, 0.0, -5.0),
//     );
//     spawn_procedural_ore(
//         &mut commands,
//         &mut meshes,
//         &mut materials,
//         Vec3::new(0.0, 0.0, 5.0),
//     );
// }
//
// pub fn spawn_procedural_tree(
//     commands: &mut Commands,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<StandardMaterial>>,
//     position: Vec3,
// ) {
//     let trunk_material = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.4, 0.26, 0.13),
//         ..default()
//     });
//
//     let leaves_material = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.13, 0.55, 0.13),
//         ..default()
//     });
//
//     // Trunk
//     commands.spawn((
//         Mesh3d(meshes.add(Cuboid::new(0.4, 2.0, 0.4))),
//         MeshMaterial3d(trunk_material),
//         Transform::from_xyz(position.x, position.y + 1.0, position.z),
//     ));
//
//     // Leaves (stacked cones)
//     for i in 0..3 {
//         let scale = 1.0 - (i as f32 * 0.25);
//         let y_offset = 2.0 + (i as f32 * 1.0);
//         commands.spawn((
//             Mesh3d(meshes.add(Cone::new(1.5 * scale, 1.5))),
//             MeshMaterial3d(leaves_material.clone()),
//             Transform::from_xyz(position.x, position.y + y_offset, position.z),
//         ));
//     }
// }
//
// pub fn spawn_procedural_rock(
//     commands: &mut Commands,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<StandardMaterial>>,
//     position: Vec3,
// ) {
//     let rock_material = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.4, 0.4, 0.45),
//         perceptual_roughness: 0.9,
//         ..default()
//     });
//
//     // Main rock body
//     commands.spawn((
//         Mesh3d(meshes.add(Sphere::new(1.0).mesh().ico(2).unwrap())),
//         MeshMaterial3d(rock_material.clone()),
//         Transform::from_xyz(position.x, position.y + 0.8, position.z)
//             .with_scale(Vec3::new(1.2, 0.8, 1.0)),
//     ));
//
//     // Smaller rocks around
//     for i in 0..3 {
//         let angle = (i as f32 / 3.0) * std::f32::consts::TAU;
//         let radius = 1.5;
//         let x = position.x + angle.cos() * radius;
//         let z = position.z + angle.sin() * radius;
//         let scale = 0.3 + (i as f32 * 0.1);
//
//         commands.spawn((
//             Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(1).unwrap())),
//             MeshMaterial3d(rock_material.clone()),
//             Transform::from_xyz(x, position.y + scale * 0.3, z).with_scale(Vec3::splat(scale)),
//         ));
//     }
// }
//
// pub fn spawn_procedural_ore(
//     commands: &mut Commands,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<StandardMaterial>>,
//     position: Vec3,
// ) {
//     let ore_material = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.8, 0.2, 0.2),
//         emissive: LinearRgba::rgb(0.3, 0.0, 0.0),
//         metallic: 0.8,
//         perceptual_roughness: 0.3,
//         ..default()
//     });
//
//     let stone_material = materials.add(StandardMaterial {
//         base_color: Color::srgb(0.35, 0.35, 0.4),
//         perceptual_roughness: 0.95,
//         ..default()
//     });
//
//     // Stone base
//     commands.spawn((
//         Mesh3d(meshes.add(Cuboid::new(1.5, 1.0, 1.5))),
//         MeshMaterial3d(stone_material),
//         Transform::from_xyz(position.x, position.y + 0.5, position.z),
//     ));
//
//     // Ore veins
//     for i in 0..4 {
//         let angle = (i as f32 / 4.0) * std::f32::consts::TAU;
//         let offset = 0.4;
//         let x = position.x + angle.cos() * offset;
//         let z = position.z + angle.sin() * offset;
//
//         commands.spawn((
//             Mesh3d(meshes.add(Cuboid::new(0.3, 0.8, 0.3))),
//             MeshMaterial3d(ore_material.clone()),
//             Transform::from_xyz(x, position.y + 0.5, z).with_rotation(Quat::from_rotation_y(angle)),
//         ));
//     }
// }
//
// pub fn setup_lighting(mut commands: Commands) {
//     // Directional light (sun)
//     commands.spawn((
//         DirectionalLight {
//             illuminance: 1500.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         Transform::from_rotation(Quat::from_euler(
//             EulerRot::XYZ,
//             -45.0f32.to_radians(),
//             45.0f32.to_radians(),
//             0.0,
//         )),
//     ));
// }
//
// pub fn setup_camera(mut commands: Commands, mut camera_state: ResMut<CameraState>) {
//     camera_state.focus = Vec3::ZERO;
//     camera_state.radius = 30.0;
//     camera_state.pitch = -45.0f32.to_radians();
//     camera_state.yaw = 45.0f32.to_radians();
//
//     commands.spawn((
//         Camera3d::default(),
//         Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
//         MainCamera,
//     ));
// }
//
// pub fn camera_controller(
//     mut camera_query: Query<&mut Transform, With<MainCamera>>,
//     mut camera_state: ResMut<CameraState>,
//     accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
//     accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
//     keys: Res<ButtonInput<KeyCode>>,
// ) {
//     let sensitivity = 0.005;
//     let zoom_sensitivity = 0.002;
//     let pan_sensitivity = 0.01;
//
//     // Handle mouse motion for orbiting and panning
//     let delta = accumulated_mouse_motion.delta;
//
//     if camera_state.is_orbiting {
//         // Orbit: change pitch and yaw
//         camera_state.yaw -= delta.x * sensitivity;
//         camera_state.pitch += delta.y * sensitivity;
//         // Clamp pitch to prevent flipping
//         camera_state.pitch = camera_state
//             .pitch
//             .clamp(-89f32.to_radians(), 89f32.to_radians());
//     } else if camera_state.is_panning {
//         // Pan: move focus point
//         let radius = camera_state.radius;
//         let right = Vec3::new(camera_state.yaw.cos(), 0.0, -camera_state.yaw.sin());
//         let up = Vec3::Y;
//         camera_state.focus -= right * delta.x * pan_sensitivity * radius;
//         camera_state.focus += up * delta.y * pan_sensitivity * radius;
//     }
//
//     // Handle mouse wheel for zooming
//     if accumulated_mouse_scroll.delta.y != 0.0 {
//         let zoom = accumulated_mouse_scroll.delta.y * zoom_sensitivity * camera_state.radius;
//         camera_state.radius = (camera_state.radius - zoom).clamp(2.0, 200.0);
//     }
//
//     // Keyboard numpad for quick view alignment (like Blender)
//     if keys.just_pressed(KeyCode::Numpad1) {
//         camera_state.pitch = 0.0;
//         camera_state.yaw = -90f32.to_radians(); // Front view
//     }
//     if keys.just_pressed(KeyCode::Numpad3) {
//         camera_state.pitch = 0.0;
//         camera_state.yaw = 0.0; // Right view
//     }
//     if keys.just_pressed(KeyCode::Numpad7) {
//         camera_state.pitch = 90f32.to_radians();
//         camera_state.yaw = 0.0; // Top view
//     }
//
//     // Update camera position based on orbit parameters
//     for mut transform in camera_query.iter_mut() {
//         let pos = camera_state.focus
//             + Vec3::new(
//                 camera_state.radius * camera_state.pitch.cos() * camera_state.yaw.cos(),
//                 camera_state.radius * camera_state.pitch.sin(),
//                 camera_state.radius * camera_state.pitch.cos() * camera_state.yaw.sin(),
//             );
//         transform.translation = pos;
//         *transform = transform.looking_at(camera_state.focus, Vec3::Y);
//     }
// }
//
// pub fn mouse_button_input(
//     mut camera_state: ResMut<CameraState>,
//     mouse_input: Res<ButtonInput<MouseButton>>,
//     keys: Res<ButtonInput<KeyCode>>,
// ) {
//     // Middle mouse button = orbit
//     if mouse_input.just_pressed(MouseButton::Middle) {
//         if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
//             camera_state.is_panning = true;
//         } else {
//             camera_state.is_orbiting = true;
//         }
//     }
//
//     if mouse_input.just_released(MouseButton::Middle) {
//         camera_state.is_orbiting = false;
//         camera_state.is_panning = false;
//     }
// }
