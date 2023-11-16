use bevy::app::PluginGroupBuilder;
use bevy::log::LogPlugin;
use bevy::{prelude::*, window::WindowResized};

pub mod statistics;

fn make_visible(mut window: Query<&mut Window>) {
    window.single_mut().visible = true;
}
fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for event in resize_reader.read() {
        info!("window resized: {:?}", event);
    }
}

fn main() {
    let app_primary_window = Window {
        title: "TakiApp".into(),
        resolution: (460., 370.).into(),
        visible: false,
        ..default()
    };
    let ow_window_plugin = WindowPlugin {
        primary_window: Some(app_primary_window),
        ..default()
    };
    let ow_log_plugin = LogPlugin {
        level: bevy::log::Level::INFO,
        ..default()
    };
    let ow_default_plugins = DefaultPlugins.set(ow_window_plugin).set(ow_log_plugin);

    App::new()
        .add_systems(Startup, make_visible)
        .add_systems(Update, on_resize_system)
        .add_plugins((
            ow_default_plugins,
            TakiAppPlugins,
            statistics::TakiStatisticsPlugins,
        ))
        .run();
}

pub struct TakiAppPlugins;
impl PluginGroup for TakiAppPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(TakiAppDemoPlugin)
            .add(materials::TakiMaterialPlugin)
            .add(particles::CorePlugin)
    }
}

pub struct TakiAppDemoPlugin;
fn log_demo() {
    trace!("very noisy");
    debug!("msg for debugging");
    info!("helpful information that is worth printing by default");
    warn!("some bad happended that isn't a failure, but thats worth calling out");
    error!("something failed and we need to know about it");
}

impl Plugin for TakiAppDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, log_demo);
    }
}

pub mod materials {
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
        mut materials: ResMut<Assets<TakiMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        commands.spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 128.0 })),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            material: materials.add(TakiMaterial {
                color: Color::BLUE,
                color_texture: asset_server.load("space_shuttle.png").into(),
                alpha_mode: AlphaMode::Blend,
            }),
            ..default()
        });
    }

    pub struct TakiMaterialPlugin;
    impl Plugin for TakiMaterialPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup);
        }
    }
}

pub mod particles {
    use bevy::{
        input::{mouse::MouseButtonInput, ButtonState},
        prelude::*,
        sprite::*,
        window::{PrimaryWindow, WindowResolution},
    };

    // ################## COMPONENTS
    #[derive(Component, Clone)]
    pub struct FireworkParticle {
        velocity: Vec2,
        position: Vec2,
        // lifetime: Option<f32>, // or Timer
        lifetime: Timer,
    }

    #[derive(Component, Debug)]
    pub struct FireworkParticleSpawner {
        rate: f32,
        amount_per_burst: usize,
        lifetime: f32,
        timer: Timer,
    }

    #[derive(Component, Debug)]
    pub struct Ground;

    // ################## DOMAIN
    fn spawn_fireworks(
        commands: &mut Commands,
        pos: Vec2,
        primary_window_resolution: &WindowResolution,
    ) {
        let bird_x = (primary_window_resolution.width() / -2.);
        let bird_y = (primary_window_resolution.height() / 2.);

        let half_extets = 0.5
            * Vec2::new(
                primary_window_resolution.width(),
                primary_window_resolution.height(),
            );

        info!("Spawning Firework");
    }

    // ################## PLUGINS
    fn setup(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        asset_server: Res<AssetServer>,
    ) {
        commands.spawn(TextBundle::from_section(
            "Particles",
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

    fn update(time: Res<Time>) {}

    fn mouse_handler(
        mut commands: Commands,
        mut mousebtn_evr: EventReader<MouseButtonInput>,
        p_window: Query<&Window, With<PrimaryWindow>>,
    ) {
        for ev in mousebtn_evr.read() {
            // right and left mouse button
            match ev.state {
                ButtonState::Pressed => match p_window.single().cursor_position() {
                    Some(pos) => {
                        info!("Mouse left button pressed at {:?}", pos);
                        spawn_fireworks(&mut commands, pos, &p_window.single().resolution);
                    }
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
                .add_systems(Update, (mouse_handler, update));
        }
    }
}
