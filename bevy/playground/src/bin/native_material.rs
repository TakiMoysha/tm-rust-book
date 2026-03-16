use bevy::color::palettes::tailwind;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

const MATERIAL_SHADER_PATH: &str = "shaders/seascape.wgsl";
const MATERIAL_TEXTURE_PATH: &str = "textures/bricks1.png";

#[derive(ShaderType, Debug, Clone)]
struct MaterialParams {
    pub time_offset: f32,
    pub color: LinearRgba,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WithShaderMaterial {
    alpha_mode: AlphaMode,

    #[uniform(0)]
    pub params: MaterialParams,

    #[texture(2)]
    #[sampler(3)]
    color_texture: Option<Handle<Image>>,
}

impl Material for WithShaderMaterial {
    fn fragment_shader() -> ShaderRef {
        MATERIAL_SHADER_PATH.into()
    }

    fn vertex_shader() -> ShaderRef {
        MATERIAL_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
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
        .add_systems(Update, (rotate_object, render_materials))
        .run();
}

fn rotate_object(mut query: Query<&mut Transform, With<Mesh3d>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.01);
        transform.rotate_x(0.005);
    }
}

fn render_materials(time: Res<Time>, mut custom_material: ResMut<Assets<WithShaderMaterial>>) {
    for material in custom_material.iter_mut() {
        material.1.params.time_offset += time.delta_secs();
    }
}

fn scena_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shader_materials: ResMut<Assets<WithShaderMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(1.).mesh().uv(64, 64))),
        MeshMaterial3d(shader_materials.add(WithShaderMaterial {
            params: MaterialParams {
                time_offset: 0.,
                color: tailwind::AMBER_100.into(),
            },
            color_texture: Some(asset_server.load(MATERIAL_TEXTURE_PATH)),
            alpha_mode: AlphaMode::Blend,
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));

    commands.spawn((
        Text::new(""),
        Node {
            position_type: PositionType::Absolute,
            top: px(10.),
            left: px(10.),
            ..default()
        },
    ));
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

        commands.spawn((DirectionalLight::default(), Transform::from_xyz(0., 3., 3.)));
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

    pub struct BootstrapCamera3dPlugin;

    impl Plugin for BootstrapCamera3dPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, init_camera)
                .add_systems(Update, camera_behavior);
        }
    }
}
