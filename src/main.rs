use bevy::prelude::*;

fn main(){
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_people,add_pets))
        .add_systems(Update,(hello_world,(update_people,greet_people).chain(),greet_pets))
        .run();
}

fn hello_world() {
    println!("hello world!");
}

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);
#[derive(Component)]
struct Pet;

fn add_people(mut commands: Commands){
    commands.spawn((Person, Name("Steve".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Nicky".to_string())));
    commands.spawn((Person, Name("Hailey".to_string())));
}

fn add_pets(mut commands: Commands){
    commands.spawn((Pet, Name("Spot".to_string())));
    commands.spawn((Pet, Name("Ham".to_string())));
    commands.spawn((Pet, Name("Brutus".to_string())));
    commands.spawn((Pet, Name("Milo".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>){
    for name in &query {
        println!("Hello {}!",name.0);
    }
}
fn greet_pets(query: Query<&Name, Without<Person>>){
    print!("You must be the pets: ");
    for name in &query {
        print!(" {} ",name.0)
    }
    println!();
}

fn update_people(mut query: Query<&mut Name, With<Person>>){
    for mut name in &mut query{
        if name.0 == "Steve"{
            println!("Steve has transitioned!");
            name.0 = "Stevie".to_string();
            break;
        }
    }
}