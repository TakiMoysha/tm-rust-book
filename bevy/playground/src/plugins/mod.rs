pub mod editor;

pub mod scenes {
    use bevy::color::palettes::tailwind;
    use bevy::image::ImageLoaderSettings;
    use bevy::prelude::*;

    pub struct DemoScena3dPlugin;

    impl Plugin for DemoScena3dPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup)
                // .add_systems(Update, draw_cursor)
            ;
        }
    }

    #[derive(Component)]
    struct Ground;

    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(50., 50.))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: tailwind::EMERALD_100.into(),
                perceptual_roughness: 0.95,
                base_color_texture: Some(asset_server.load("textures/bricks1.png")),
                normal_map_texture: Some(asset_server.load_with_settings(
                    "textures/bricks1_normal.png",
                    |settings: &mut ImageLoaderSettings| settings.is_srgb = false,
                )),
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Ground,
        ));

        let grid_material = materials.add(StandardMaterial {
            base_color: tailwind::AMBER_700.into(),
            ..default()
        });

        for i in -5..=5 {
            let offset = i as f32 * 5.;

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(5., 0.05, 0.05))),
                MeshMaterial3d(grid_material.clone()),
                Transform::from_xyz(0.0, 0.01, offset),
            ));
        }

        // ======================================================================
        fill_scena(&mut commands, &mut meshes, &mut materials);
    }

    pub fn fill_scena(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) {
        let trunk_material = materials.add(StandardMaterial {
            base_color: tailwind::ZINC_400.into(),
            ..default()
        });

        let leaves_material = materials.add(StandardMaterial {
            base_color: tailwind::GREEN_600.into(),
            ..default()
        });

        let trunk_base_position = Vec3::new(-10., 0., -10.);

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.4, 2., 0.4))),
            MeshMaterial3d(trunk_material),
            Transform::from_xyz(
                trunk_base_position.x,
                trunk_base_position.y + 1.0,
                trunk_base_position.z,
            ),
        ));

        for i in 0..3 {
            let scale = 1.0 - (i as f32 * 0.25);
            let y_offset = 2.0 + (i as f32 * 1.0);
            commands.spawn((
                Mesh3d(meshes.add(Cone::new(1.4 * scale, 1.5))),
                MeshMaterial3d(leaves_material.clone()),
                Transform::from_xyz(
                    trunk_base_position.x,
                    trunk_base_position.y + y_offset,
                    trunk_base_position.z,
                ),
            ));
        }

        // ===========================================================
        let rock_base_position = vec3(15.1, 0.5, -15.);

        let rock_material = materials.add(StandardMaterial {
            base_color: tailwind::STONE_600.into(),
            perceptual_roughness: 0.8,
            ..default()
        });

        commands.spawn((
            Mesh3d(meshes.add(Sphere::new(1.2).mesh().ico(1).unwrap())),
            MeshMaterial3d(rock_material.clone()),
            Transform::from_xyz(
                rock_base_position.x,
                rock_base_position.y,
                rock_base_position.z,
            )
            .with_scale(vec3(1., 1.2, 2.)),
        ));

        for i in 0..3 {
            let angle = (i as f32 / 3.0) * std::f32::consts::TAU;
            let radius = 1.5;
            let x = rock_base_position.x + angle.cos() * radius * 1.2;
            let z = rock_base_position.z + angle.sin() * radius * 1.2;
            let scale = 0.6 + (i as f32 * 0.2);

            commands.spawn((
                Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(1).unwrap())),
                MeshMaterial3d(rock_material.clone()),
                Transform::from_xyz(x, rock_base_position.y, z)
                    .with_scale(vec3(scale, scale, scale)),
            ));
        }

        // ===========================================================
        let ore_base_position = Vec3::new(-10., 1., 10.);

        let ore_material = materials.add(StandardMaterial {
            base_color: tailwind::STONE_600.into(),
            emissive: tailwind::ROSE_800.into(),
            metallic: 0.8,
            perceptual_roughness: 0.4,
            ..default()
        });

        let stone_material = materials.add(StandardMaterial {
            base_color: tailwind::STONE_900.into(),
            perceptual_roughness: 0.95,
            ..default()
        });

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(1.5, 1.0, 1.5))),
            MeshMaterial3d(stone_material),
            Transform::from_xyz(
                ore_base_position.x,
                ore_base_position.y + 0.5,
                ore_base_position.z,
            ),
        ));

        for i in 0..rand::random_range(2..8_i32) {
            let angle = (i as f32 / 4.0) * std::f32::consts::TAU;
            let offset = 0.4;
            let x = ore_base_position.x + angle.cos() * offset;
            let z = ore_base_position.y + angle.cos() * offset;

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::new(0.3, 0.4, 0.3))),
                MeshMaterial3d(ore_material.clone()),
                Transform::from_xyz(x, ore_base_position.y, z)
                    .with_rotation(Quat::from_rotation_y(angle)),
            ));
        }
    }

    // WIP: second camera (second viewport, like minimap or split screen)
    // #[derive(Component)]
    // #[require(Camera3d)]
    // pub struct DevelopCamera;
    //
    // fn draw_cursor(
    //     camera_query: Single<(&DevelopCamera, &GlobalTransform)>,
    //     ground: Single<&GlobalTransform, With<Ground>>,
    //     window: Single<&Window>,
    //     mut gizmos: Gizmos,
    // ) {
    //     let (camera, camera_transform) = *camera_query;
    //
    //     if let Some(cursor_position) = window.cursor_position()
    //     // Calculate a ray pointing from the camera into the world based on the cursor's position.
    //     && let Ok(ray) = camera.viewport_to(camera_transform, cursor_position)
    //     // Calculate if and where the ray is hitting the ground plane.
    //     && let Some(point) = ray.plane_intersection_point(ground.translation(), InfinitePlane3d::new(ground.up()))
    //     {
    //         // Draw a circle just above the ground plane at that position.
    //         gizmos.circle(
    //             Isometry3d::new(
    //                 point + ground.up() * 0.01,
    //                 Quat::from_rotation_arc(Vec3::Z, ground.up().as_vec3()),
    //             ),
    //             0.2,
    //             Color::WHITE,
    //         );
    //     }
    // }
}

pub mod cameras {
    use bevy::prelude::*;
    use bevy_input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};

    #[derive(Resource, Debug, Default)]
    pub struct CameraState {
        focus: Vec3,
        radius: f32,
        pitch: f32,
        yaw: f32,
        is_orbiting: bool,
        is_panning: bool,
    }

    #[derive(Component)]
    #[require(Camera3d)]
    pub struct BootstrapCamera3d;

    fn setup_camera(mut commands: Commands) {
        commands.insert_resource(CameraState {
            focus: Vec3::ZERO,
            ..default()
        });

        let camera_init_position =
            Transform::from_xyz(40.0, 40.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y);
        commands.spawn((BootstrapCamera3d, camera_init_position));

        commands.spawn((DirectionalLight {
            illuminance: 1400.,
            shadows_enabled: true,
            ..default()
        },));
    }

    fn camera_behavior(
        time: Res<Time>,
        input: Res<ButtonInput<KeyCode>>,
        accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
        accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
        mut main_camera: Single<(&mut Transform, &BootstrapCamera3d)>,
        mut camera_state: Option<ResMut<CameraState>>,
        // mut light: Single<(&mut Transform, &DirectionalLight)>,
    ) {
        let dt = time.delta_secs();
        let mouse_sensitivity = Vec2::new(0.12, 0.1);
        let move_speed = 10.0;

        let mut cam_direction = Vec3::ZERO;

        if input.pressed(KeyCode::KeyW) {
            cam_direction += Vec3::new(0., 0., 1.);
        }
        if input.pressed(KeyCode::KeyS) {
            cam_direction += Vec3::new(0., 0., -1.);
        }
        if input.pressed(KeyCode::KeyA) {
            cam_direction += Vec3::new(-1., 0., 0.);
        }
        if input.pressed(KeyCode::KeyD) {
            cam_direction += Vec3::new(1., 0., 0.);
        }
        if input.pressed(KeyCode::Space) {
            cam_direction += Vec3::new(0., 1., 0.);
        }
        if input.pressed(KeyCode::KeyC) {
            cam_direction += Vec3::new(0., -1., 0.);
        }

        if cam_direction != Vec3::ZERO {
            let direction = cam_direction.normalize();
            main_camera.0.translation += direction * move_speed * dt;
        }
    }

    use bevy_input::mouse::MouseButtonInput;

    #[derive(Debug, Resource)]
    struct MousePressed(bool);

    pub fn mouse_button_input(
        mut camera_state: ResMut<CameraState>,
        mut mouse_button_inputs: MessageReader<MouseButtonInput>,
        mouse_input: Res<ButtonInput<MouseButton>>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        // Middle mouse button = orbit
        if mouse_input.just_pressed(MouseButton::Middle) {
            if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
                camera_state.is_panning = true;
            } else {
                camera_state.is_orbiting = true;
            }
        }

        if mouse_input.just_released(MouseButton::Middle) {
            camera_state.is_orbiting = false;
            camera_state.is_panning = false;
        }

        for mouse_button_input in mouse_button_inputs.read() {
            if mouse_button_input.button == MouseButton::Right {
                // *mouse_pressed = MousePressed(mouse_button_input.state.is_pressed());
            }
        }
    }

    pub fn camera_controller(
        mut camera_query: Query<&mut Transform, With<BootstrapCamera3d>>,
        mut camera_state: ResMut<CameraState>,
        accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
        accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        let mouse_sensitivity = Vec2::new(0.12, 0.1);
        let zoom_sensitivity = 0.002;
        let pan_sensitivity = 0.01;

        // Handle mouse motion for orbiting and panning
        let delta = accumulated_mouse_motion.delta;

        if camera_state.is_orbiting {
            // Orbit: change pitch and yaw
            camera_state.yaw -= delta.x * mouse_sensitivity.x;
            camera_state.pitch += delta.y * mouse_sensitivity.x;
            // Clamp pitch to prevent flipping
            camera_state.pitch = camera_state
                .pitch
                .clamp(-89f32.to_radians(), 89f32.to_radians());
        } else if camera_state.is_panning {
            // Pan: move focus point
            let radius = camera_state.radius;
            let right = Vec3::new(camera_state.yaw.cos(), 0.0, -camera_state.yaw.sin());
            let up = Vec3::Y;
            camera_state.focus -= right * delta.x * pan_sensitivity * radius;
            camera_state.focus += up * delta.y * pan_sensitivity * radius;
        }

        // Handle mouse wheel for zooming
        if accumulated_mouse_scroll.delta.y != 0.0 {
            let zoom = accumulated_mouse_scroll.delta.y * zoom_sensitivity * camera_state.radius;
            camera_state.radius = (camera_state.radius - zoom).clamp(2.0, 200.0);
        }

        // Keyboard numpad for quick view alignment (like Blender)
        if keys.just_pressed(KeyCode::Digit1) {
            camera_state.pitch = 0.0;
            camera_state.yaw = -90f32.to_radians(); // Front view
        }
        if keys.just_pressed(KeyCode::Digit2) {
            camera_state.pitch = 0.0;
            camera_state.yaw = 0.0; // Right view
        }
        if keys.just_pressed(KeyCode::Digit3) {
            camera_state.pitch = 90f32.to_radians();
            camera_state.yaw = 0.0; // Top view
        }

        // Update camera position based on orbit parameters
        for mut transform in camera_query.iter_mut() {
            let pos = camera_state.focus
                + Vec3::new(
                    camera_state.radius * camera_state.pitch.cos() * camera_state.yaw.cos(),
                    camera_state.radius * camera_state.pitch.sin(),
                    camera_state.radius * camera_state.pitch.cos() * camera_state.yaw.sin(),
                );
            transform.translation = pos;
            *transform = transform.looking_at(camera_state.focus, Vec3::Y);
        }
    }

    pub struct BootstrapCamera3dPlugin;

    impl Plugin for BootstrapCamera3dPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup_camera).add_systems(
                Update,
                (
                    camera_behavior,
                    // mouse_button_input,
                    // camera_controller
                ),
            );
        }
    }
}
