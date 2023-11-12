use bevy::prelude::*;

// Entity Component System
#[derive(Component)]
pub struct Name(String);

#[derive(Component)]
pub struct Person;

pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("JoJo".to_string())));
}

#[derive(Resource)]
pub struct GreetTimer(Timer);

pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
            println!("time: {}", time.elapsed_seconds());
        }
    }
}

pub struct StaterdDefaultPlugins;

impl Plugin for StaterdDefaultPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, greet_people);
    }
}

