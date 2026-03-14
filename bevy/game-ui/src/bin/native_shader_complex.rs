use bevy::color::palettes::tailwind;
use bevy::math::VectorSpace;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

const DEBUG_SHADER_ASSET_PATH: &str = "shaders/simple.wgsl";
const SHADER_ASSET_PATH: &str = "shaders/seascape.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WithShaderMaterial {}

impl Material for WithShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        // DEBUG_SHADER_ASSET_PATH.into()
        SHADER_ASSET_PATH.into()
    }

    fn vertex_shader() -> ShaderRef {
        // DEBUG_SHADER_ASSET_PATH.into()
        SHADER_ASSET_PATH.into()
    }
}

fn main() {
    let _matches = clap::command!()
        .arg(clap::arg!(--test "run simple test_scena").action(clap::ArgAction::SetTrue))
        .get_matches();

    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<WithShaderMaterial>::default(),
            camera_plugin::BootstrapCamera3dPlugin,
        ))
        .add_systems(Startup, scena_setup)
        .add_systems(Update, rotate_object)
        .run();
}

fn rotate_object(mut query: Query<&mut Transform, With<Mesh3d>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.01);
        transform.rotate_x(0.005);
    }
}

fn scena_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shader_materials: ResMut<Assets<WithShaderMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    // commands.spawn((
    //     Mesh3d(meshes.add(Circle::new(4.0))),
    //     MeshMaterial3d(materials.add(tailwind::EMERALD_700.into())),
    //     Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    // ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(standard_materials.add(Color::from(tailwind::ZINC_700))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn(DirectionalLight {
        shadows_enabled: true,
        ..default()
    });
}

mod camera_plugin {
    use bevy::prelude::*;
    use bevy_input::mouse::MouseMotion;

    #[derive(Component)]
    #[require(Camera3d)]
    pub struct BootstrapCamera3d;

    fn init_camera(mut commands: Commands) {
        commands.spawn((
            BootstrapCamera3d,
            Transform::from_xyz(-2.4, 3., 9.).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }

    fn camera_behavior(
        time: Res<Time>,
        input: Res<ButtonInput<KeyCode>>,
        mut mouse_motion: MessageReader<MouseMotion>,
        mut camera: Single<(&mut Transform, &BootstrapCamera3d)>,
    ) {
        let dt = time.delta_secs();
        let mouse_sensitivity = Vec2::new(0.12, 0.1);
        let move_speed = 10.0;

        let mut direction = Vec3::ZERO;

        if input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0., 0., 1.);
        }
        if input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0., 0., -1.);
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
            info!("direction: {}", direction);
            let direction = direction.normalize();
            Transform::from_translation(camera.0.translation);
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
    }

    pub struct BootstrapCamera3dPlugin;

    impl Plugin for BootstrapCamera3dPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, init_camera)
                .add_systems(Update, camera_behavior);
        }
    }
}
