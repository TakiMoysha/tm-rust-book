use bevy::{prelude::*, render::render_resource::AsBindGroup};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TakiMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

impl Material for TakiMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "shaders/taki_simple.material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}

pub fn ground() -> StandardMaterial {
    StandardMaterial {
        base_color: Color::rgb(0.3, 0.5, 0.3),
        perceptual_roughness: 1.0,
        metallic: 0.0,
        ..default()
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<TakiMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 128.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(StandardMaterial {
            base_color: Color::rgb(0.3, 0.5, 0.3),
            perceptual_roughness: 1.0,
            metallic: 0.0,
            ..default()
        }),
        // material: materials.add(TakiMaterial {
        //     color: Color::BLUE,
        //     color_texture: asset_server.load("space_shuttle.png").into(),
        //     alpha_mode: AlphaMode::Blend,
        // }),
        ..default()
    });
}

pub struct TakiMaterialPlugin;
impl Plugin for TakiMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

