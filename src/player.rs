use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
struct Player;
#[derive(Component)]
struct Player_Coords;
#[derive(Component)]
struct Player_Velocity;

fn generate_player(){
    //spawn camera and player hitbox
    //attach camera to player
}

fn player_movement(){
    //generate WASD move + space jump to modify player position
}

fn update_player_pos(){
    //have movement keys jsut modify variable coord then have this rerawn
    //use FixedUpdate system rate 
}
