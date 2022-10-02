use bevy::prelude::*;


// Components
#[derive(Component)]
struct Position { x: f32, y: f32 }

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;


// Systems
fn print_position_system(query: Query<&Transform>) {
    for transform in query.iter() {
        println!("position: {:?}", transform.translation);
    }
}

fn hello_world() {
    println!("hello world!");
}

fn add_person(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Mishko".to_string()));
    commands.spawn().insert(Person).insert(Name("Ne Mishko".to_string()));
}

fn greet_person(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0)
    }
}

// Entities
struct Entity(u64);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_person)
        .add_system(hello_world)
        .add_system(greet_person)
        .run();
}
