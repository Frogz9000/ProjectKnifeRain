use crate::{level_management::generate_debug_world::GenerateDebugWorldPlugin, player::SpawnInfo};
use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query},
};
use bevy::prelude::*;
pub fn cleanup_with_component<T: Component>(mut command: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        command.entity(e).despawn();
    }
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum LevelState {
    #[default]
    NoLevel,
    //static levels
    DebugWorld,
    //Overworld,
    //Shop,
    //Town, etc
    //
    //procedural
    //ProceduralLevel { seed: i64 }//replace with actual seed later
    //could have
    //ForrestDungeon { seed: i64 },
    //CaveDungeon { seed: i64 }, etc
    //to have several stored rand dungeons that only get cleared with
    //certain conditions
}

pub struct WorldController;
impl Plugin for WorldController {
    fn build(&self, app: &mut App) {
        app.init_state::<LevelState>();
        app.add_event::<SpawnPlayerEvent>();
        app.add_plugins(GenerateDebugWorldPlugin);
        app.add_systems(Startup, start_debug_world);
        app.add_systems(Update, reset_debug_world);
        app.add_systems(FixedUpdate, start_debug_world.run_if(in_state(LevelState::NoLevel)));

    }
}
#[derive(Event)]
pub struct SpawnPlayerEvent {
    pub(crate) spawn_info: SpawnInfo,
    //is_host: bool,//determin if spawning player character or multiplayer entity
    //TODO: have other player moved to player.rs from netcode and just have an
    //is_host if statement go to a different command block
}

fn start_debug_world(
    mut next_state: ResMut<NextState<LevelState>>,
) {
    next_state.set(LevelState::DebugWorld);
}



fn reset_debug_world(
    mut next_state: ResMut<NextState<LevelState>>,
    temp_input: Res<ButtonInput<KeyCode>>,
) {
    //have menu button controlled later, for now R resets debug
    if temp_input.pressed(KeyCode::KeyR){
        next_state.set(LevelState::NoLevel);
    }
}