//offline documentation of current project
//cargo doc --open

use bevy::prelude::*;

#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);
#[derive(Component)]
struct Pet;
#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;
impl Plugin for HelloPlugin{
    fn build(&self, app: &mut App){
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, (add_people,add_pets));
        app.add_systems(Update,(update_people,greet_all).chain());
    }
}

fn hello_world() {
    println!("hello world!");
}

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
//replace 
//app.add_systems(Startup, (add_people,add_pets));
//app.add_systems(Update,((update_people,greet_people).chain(),greet_pets));
//with just greet_all in plugin def
fn greet_all(time: Res<Time>,
            mut timer: ResMut<GreetTimer>,
            person_query: Query<&Name, With<Person>>,
            pet_query: Query<&Name, Without<Person>>){
    if timer.0.tick(time.delta()).just_finished(){
        for name in &person_query {
        println!("Hello {}!",name.0);
        }
        print!("You must be the pets: ");
        for name in &pet_query {
        print!(" {} ",name.0)
        }
        println!();
    }
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