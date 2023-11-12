pub mod particles {
    use bevy::{
        input::{mouse::MouseButtonInput, ButtonState},
        prelude::*,
        window::PrimaryWindow,
    };

    #[derive(Component, Clone)]
    pub struct Particle {
        velocity: Vec2,
        position: Vec2,
        // lifetime: Option<f32>, // or Timer
        lifetime: Timer,
    }

    #[derive(Component, Debug)]
    pub struct FireworksParticleSpawner {
        rate: f32,
        amount_per_burst: usize,
        lifetime: f32,
        timer: Timer,
    }

    #[derive(Component, Debug)]
    pub struct Ground;

    // DOMAIN
    fn spawn_fireworks(
        commands: &mut Commands,
        pos: Vec2,
        mut spawners: Query<(Entity, &mut FireworksParticleSpawner)>,
        time: Res<Time>,
    ) {
        for (ent, mut spawner) in spawners.iter_mut() {
            println!("spawner: {:?}", spawner);
            spawner.timer.tick(time.delta());
            println!("timer: {:?}", spawner.timer);
            // if spawner.timer.just_finished() {
            // println!("spawning");
            // for index in 0..spawner.amount_per_burst {
            //     let particle = Particle {
            //         velocity: Vec2::Y * -1.,
            //         position: pos,
            //         lifetime: Timer::from_seconds(spawner.lifetime, TimerMode::Once),
            //     };
            //     let particle = commands.spawn_empty().insert(particle).id();
            //     let mut sprite = SpriteBundle::default();
            //     // commands.entity(ent).add_child(particle);
            // }
            // }
        }
    }

    // PLUGINS
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
                mesh: sphere_handle.clone(),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
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
            match ev.state {
                ButtonState::Pressed => match p_window.single().cursor_position() {
                    Some(pos) => {
                        println!("Mouse left button pressed at {:?}", pos);
                        // spawn_fireworks(&mut commands, pos);
                    }
                    None => {
                        println!("Mouse left button pressed, but no cursor position available");
                    }
                },
                ButtonState::Released => println!("Mouse left button released"),
            }
        }
    }

    fn draw_cursor(
        camera_query: Query<(&Camera, &GlobalTransform)>,
        ground_query: Query<&GlobalTransform, With<Ground>>,
        windows: Query<&Window>,
        mut gizmos: Gizmos,
    ) {
        let (camera, camera_transform) = camera_query.single();
        let ground = ground_query.single();
        let Some(cursor_position) = windows.single().cursor_position() else {
            return;
        };
        let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
            return;
        };
        let Some(distance) = ray.intersect_plane(ground.translation(), ground.up()) else {
            return;
        };
        let point = ray.get_point(distance);
        gizmos.circle(point + ground.up() * 3., ground.up(), 0.1, Color::WHITE);
    }

    // SUPPORT
    pub struct CorePlugins;
    impl Plugin for CorePlugins {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup)
                .add_systems(Update, (mouse_handler, update, draw_cursor));
        }
    }
}

use bevy::{prelude::*, window::WindowResized};

fn make_visible(mut window: Query<&mut Window>) {
    window.single_mut().visible = true;
}
fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for event in resize_reader.read() {
        println!("window resized: {:?}", event);
    }
}

fn main() {
    let app_primary_window = Window {
        title: "TakiApp".into(),
        resolution: (460., 370.).into(),
        visible: false,
        ..default()
    };
    let overwrite_defaul_plugin = WindowPlugin {
        primary_window: Some(app_primary_window),
        ..default()
    };
    App::new()
        .add_systems(Startup, make_visible)
        .add_systems(Update, on_resize_system)
        .add_plugins((
            DefaultPlugins.set(overwrite_defaul_plugin),
            particles::CorePlugins,
        ))
        .run();
}
