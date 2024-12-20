use bevy::app::PluginGroupBuilder;
use bevy::log::LogPlugin;
use bevy::{prelude::*, window::WindowResized};

pub mod demo;
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
            .add(particles::CorePlugin)
    }
}

pub mod particles {
    use bevy::{
        input::{mouse::MouseButtonInput, ButtonState},
        prelude::*,
        window::{PrimaryWindow, WindowResolution},
    };
    // ################## COMPONENTS
    #[derive(Component, Clone)]
    pub struct Lifetime {
        timer: Timer,
    }

    // A particle is the visible part of a particle system.
    // It's what you see on the screen when a particle system is active:
    // The tiny specks of dust, the flames of a fire, the glowing orbs of a magical effect.
    // You can have anywhere between a couple hundred and tens of thousands of particles in a single system.
    // You can randomize a particle's size, its speed and movement direction,
    // and change its color over the course of its lifetime.
    #[derive(Component, Clone, Reflect, Debug, Default)]
    pub struct Particle {
        pub lifetime: Timer,
    }

    #[derive(Component, Default, Clone)]
    pub struct ParticleDesc {
        pub particle: Particle,
        pub sprite: SpriteSheetBundle,
        pub falling: Option<FallingParticle>,
        pub radial: Option<RadialParticle>,
        pub rotation: Option<RotationParticle>,
        pub fading: Option<FadingParticle>,
    }

    #[derive(Component, Reflect)]
    pub struct ParticleParent;

    // An emitter is what's creating the particles.
    // Emitters are usually not visible, but they can have a shape.
    // That shape controls where and how particles are spawned,
    // for example whether they should fill a room like dust or shoot away from a single point like a fountain.
    #[derive(Component, Clone)]
    pub struct RectParticleEmitter {
        pub particle_parent: Entity,
        pub size: Vec2,
        pub rate: Timer,
        pub varients: usize,
        pub desc: ParticleDesc,
    }

    #[derive(Component, Clone)]
    pub struct FallingParticle {
        pub speed: f32,
    }
    #[derive(Component, Clone)]
    pub struct RadialParticle {
        pub speed: f32,
    }
    #[derive(Component, Clone)]
    pub struct RotationParticle {
        pub speed: f32,
    }
    #[derive(Component, Clone)]
    pub struct FadingParticle {
        pub speed: f32,
    }
    // ################## ANIMATIONS
    // fn particles_fall(
    //     mut particles: Query<(&mut Transform, &mut FallingParticle), With<Particle>>,
    //     time: Res<Time>,
    // ) {
    //     for (mut transform, falling) in &mut particles {
    //         transform.translation.y -= falling.speed * time.delta_seconds();
    //     }
    // }
    //
    // fn particles_radial(
    //     mut particles: Query<(&mut Transform, &RadialParticle), With<Particle>>,
    //     time: Res<Time>,
    // ) {
    //     for (mut transform, radial) in &mut particles {
    //         let direction = transform.translation.truncate().normalize();
    //         transform.translation += (radial.speed * time.delta_seconds()) * direction.extend(0.0);
    //     }
    // }
    //
    // fn particles_fade(mut particles: Query<(&mut Sprite, &Particle), With<FadingParticle>>) {
    //     for (mut sprite, particle) in &mut particles {
    //         sprite.color.set_a(particle.lifetime.percent_left());
    //     }
    // }
    //
    // fn particles_rotate(
    //     mut particles: Query<(&mut Transform, &RotationParticle), With<Particle>>,
    //     time: Res<Time>,
    // ) {
    //     for (mut transform, rotation) in &mut particles {
    //         transform.rotation *= Quat::from_rotation_z(rotation.speed * time.delta_seconds());
    //     }
    // }
    // #################### SYSTEMS
    fn particles_update(
        mut commands: Commands,
        mut particles: Query<(&Particle)>,
        p_window: Query<&Window, (With<PrimaryWindow>)>,
    ) {
        for particle in &mut particles {
            // if (particle.pos.x < -10 || particle.pos.x > p_window.single().resolution.height() + 10.) {
            //     commands.entity(particle).despawn();
            // } else if (particle.pos.y < -10 || particle.pos.y > p_window.single().resolution.height() + 10.) {
            //     commands.entity(entity).despawn();
            // }
        }
    }

    fn lifetimes_update(
        mut commands: Commands,
        time: Res<Time>,
        mut lifetimes: Query<(Entity, &mut Lifetime)>,
    ) {
        for (entity, mut lifetime) in &mut lifetimes {
            lifetime.timer.tick(time.delta());
            if lifetime.timer.finished() {
                commands.entity(entity).despawn();
            }
        }
    }

    fn mouse_handler(
        mut commands: Commands,
        mut mousebtn_evr: EventReader<MouseButtonInput>,
        p_window: Query<&Window, With<PrimaryWindow>>,
    ) {
        for ev in mousebtn_evr.read() {
            // right and left mouse button
            match ev.state {
                ButtonState::Pressed => match p_window.single().cursor_position() {
                    Some(pos) => {}
                    None => {
                        error!("Mouse left button pressed, but no cursor position available");
                    }
                },
                ButtonState::Released => debug!("Mouse left button released"),
            }
        }
    }

    fn setup(
        mut commands: Commands,
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
    }

    // ################## SUPPORT
    pub struct CorePlugin;

    impl Plugin for CorePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Startup, setup)
                .add_systems(Update, (mouse_handler, lifetimes_update));
        }
    }
}
