use bevy::{
    app::PluginGroupBuilder,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    text::Text,
};

#[derive(Component)]
struct StatsText;
fn init_frame_rate(mut cmds: Commands) {
    let text_section = move |color, value: &str| TextSection {
        value: value.to_string(),
        style: TextStyle {
            font_size: 16.,
            color,
            ..default()
        },
    };
    cmds.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::End,
            padding: UiRect::all(Val::Px(4.)),
            ..default()
        },
        z_index: ZIndex::Global(i32::MAX),
        background_color: Color::BLACK.with_a(0.5).into(),
        ..default()
    })
    .with_children(|c| {
        c.spawn((
            TextBundle::from_sections([
                text_section(Color::GREEN, "FPS: "),
                text_section(Color::CYAN, "\ntest"),
            ]),
            StatsText,
        ));
    });
}
fn frame_rate_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<StatsText>>,
) {
    let mut text = query.single_mut();

    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            text.sections[0].value = format!("Average: {average:.2}");
        }
    }
}

// #################### PLUGINS
struct TakiFrameTimePlugin;
impl Plugin for TakiFrameTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_frame_rate)
            .add_systems(Update, frame_rate_system);
    }
}

// #################### GROUPS
pub struct TakiStatisticsPlugins;
impl PluginGroup for TakiStatisticsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(FrameTimeDiagnosticsPlugin::default())
            .add(TakiFrameTimePlugin)
    }
}

