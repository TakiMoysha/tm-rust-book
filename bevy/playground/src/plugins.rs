pub mod scenes {
    use bevy::color::palettes::tailwind;
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
                base_color: tailwind::EMERALD_400.info(),
                base_color_texture: Some(asset_server.load("textures/dirt1.png")),
                ..default()
            })),
            Ground,
        ));
        commands.spawn((
            DirectionalLight::default(),
            Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ));

        let grid_material = materials.add(StandardMaterial {
            base_color: tailwind::GRAY_400.into(),
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        let stone_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.35, 0.35, 0.4),
            perceptual_roughness: 0.95,
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
        fill_scena(commands);
    }

    pub fn fill_scena(mut commands: Commands) {}

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
    use bevy::log::{error, info};
    use bevy::prelude::*;
    use bevy_input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseMotion};

    #[derive(Component)]
    #[require(Camera3d)]
    pub struct BootstrapCamera3d;

    #[derive(Resource, Debug, Default)]
    pub struct CameraState {
        focus: Vec3,
        radius: f32,
        pitch: f32,
        yaw: f32,
        is_orbiting: bool,
        is_panning: bool,
    }

    fn setup_camera(mut commands: Commands, mut camera_state: Option<ResMut<CameraState>>) {
        if let Some(camera_state) = &mut camera_state {
            camera_state.focus = Vec3::ZERO;
            camera_state.radius = 30.0;
            camera_state.pitch = -45.0f32.to_radians();
            camera_state.yaw = 45.0f32.to_radians();
            camera_state.is_orbiting = false;
            camera_state.is_panning = false;
        } else {
            commands.insert_resource(CameraState {
                focus: Vec3::ZERO,
                radius: 30.0,
                pitch: -45.0f32.to_radians(),
                yaw: 45.0f32.to_radians(),
                is_orbiting: false,
                is_panning: false,
            });
        }

        commands.spawn((
            BootstrapCamera3d,
            Transform::from_xyz(20.0, 20.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ));

        commands.spawn((
            DirectionalLight {
                illuminance: 1400.,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_rotation(Quat::from_euler(
                EulerRot::XYZ,
                -45.0f32.to_radians(),
                45.0f32.to_radians(),
                0.0,
            )),
        ));
    }

    fn camera_behavior(
        time: Res<Time>,
        input: Res<ButtonInput<KeyCode>>,
        mut mouse_motion: MessageReader<MouseMotion>,
        mut camera: Single<(&mut Transform, &BootstrapCamera3d)>,
        // mut light: Single<(&mut Transform, &DirectionalLight)>,
    ) {
        let dt = time.delta_secs();
        let mouse_sensitivity = Vec2::new(0.12, 0.1);
        let move_speed = 10.0;

        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0., 0., -1.);
        }
        if input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0., 0., 1.);
        }
        if input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1., 0., 0.);
        }
        if input.pressed(KeyCode::Space) {
            direction += Vec3::new(0., 1., 0.);
        }
        if input.pressed(KeyCode::KeyC) {
            direction += Vec3::new(0., -1., 0.);
        }

        if direction != Vec3::ZERO {
            let direction = direction.normalize();
            camera.0.translation += direction * move_speed * dt;
        }

        for motion_event in mouse_motion.read() {
            let delta_yaw = -motion_event.delta.x * dt * mouse_sensitivity.x;
            let delta_pitch = -motion_event.delta.y * dt * mouse_sensitivity.y;
            // let delta = Vec2::new(delta.x, delta.y) * mouse_sensitivity;
            // let yaw = Quat::from_rotation_y(-delta.x);
            // let pitch = Quat::from_rotation_x(-delta.y);
            // let rotation = yaw * pitch;
            // let mut transform = camera.single_mut();
        }
        // let light_direction = light.0.translation.normalize_or(Vec3::ZERO);
        // light.0.looking_at(Vec3::ZERO, Vec3::ONE);
    }

    pub fn mouse_button_input(
        mut camera_state: ResMut<CameraState>,
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
    }
    pub fn camera_controller(
        mut camera_query: Query<&mut Transform, With<BootstrapCamera3d>>,
        mut camera_state: ResMut<CameraState>,
        accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
        accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
        keys: Res<ButtonInput<KeyCode>>,
    ) {
        let sensitivity = 0.005;
        let zoom_sensitivity = 0.002;
        let pan_sensitivity = 0.01;

        // Handle mouse motion for orbiting and panning
        let delta = accumulated_mouse_motion.delta;

        if camera_state.is_orbiting {
            // Orbit: change pitch and yaw
            camera_state.yaw -= delta.x * sensitivity;
            camera_state.pitch += delta.y * sensitivity;
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
        if keys.just_pressed(KeyCode::Numpad1) {
            camera_state.pitch = 0.0;
            camera_state.yaw = -90f32.to_radians(); // Front view
        }
        if keys.just_pressed(KeyCode::Numpad3) {
            camera_state.pitch = 0.0;
            camera_state.yaw = 0.0; // Right view
        }
        if keys.just_pressed(KeyCode::Numpad7) {
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
            app.add_systems(Startup, setup_camera)
                .add_systems(Update, (
                    camera_behavior,
                    // camera_controller,
                    // mouse_button_input,
                ))
            ;
        }
    }
}
