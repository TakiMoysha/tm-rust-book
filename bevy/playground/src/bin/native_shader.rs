use bevy::{
    prelude::*, reflect::TypePath, render::render_resource::AsBindGroup, shader::ShaderRef,
};

const SHADER_ASSET_PATH: &str = "shaders/simple.wgsl";

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MaterialPlugin::<CustomMaterial>::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

fn rotate_cube(mut query: Query<&mut Transform, With<Mesh3d>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.01);
        transform.rotate_x(0.005);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(CustomMaterial {})),
        Transform::from_xyz(0., 0.5, 0.),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
