//player_class_actions will depend on player but player will not having only 1 way binding
use bevy_rapier3d::prelude::{Collider, GravityScale, LockedAxes, RigidBody, Velocity};
use bevy::
    prelude::*
;
use crate::player::*;

pub struct WizardPlugin;
impl Plugin for WizardPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup,(
            shoot_fireball_forward,
        ));
    }
}

//fireball object will be dynamic rigid body
//set GravityScale(0.0) to get rid of drop

fn shoot_fireball_forward(
    mut command: Commands,
    spawner_query: Query<&GlobalTransform, With<SpawnerMuzzle>>,
){
    let Some(spawn_info) = get_spawner_look_dir_and_pos(spawner_query) else {return};
    command.spawn((
        RigidBody::Dynamic,
        GravityScale(0.0),
        spawn_info.to_transform(),
    ));
}