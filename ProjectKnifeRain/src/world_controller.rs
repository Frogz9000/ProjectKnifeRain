use crate::generate_debug_world::{self, *};
use bevy::ecs::{
    component::Component,
    entity::Entity,
    query::With,
    system::{Commands, Query},
};
use bevy::prelude::*;
use bevy_rapier3d::parry::query::{self, point::PointCompositeShapeProjBestFirstVisitor};

pub fn cleanup_with_component<T: Component>(mut command: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        command.entity(e).despawn();
    }
}

pub struct WorldController;
impl Plugin for WorldController {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnPlayerEvent>();
        app.add_event::<SpawnLevelEvent>();
        app.add_systems(Startup, start_world_manager);
    }
}

#[derive(Event)]
pub struct SpawnPlayerEvent {
    pub spawn_point: Vec3, //spawn point at x, y, z in level
    pub look_dir: Vec3,    //Vec3 to look_at
                           //eventually add saveData here to load inventory for player or maybe
                           //handle that elsewhere
}
//to impliment later, ask sammy how he wants it
//#[derive(Event)]
//pub struct SpawnMuliplayerEvent {
//send event to network section to spawn in other hitboxes
//}
#[derive(Event)]
pub struct SpawnLevelEvent {}
#[derive(Event)]
pub struct CleanupLevelEvent {}

fn start_world_manager(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //if (no level to load){
    setup_test_level(commands, meshes, materials);
    //else{
    //load_level()
    //}
}

//for now this jsut clears debug stage
//later have interface that each world impls to have clear and spawn
//guaranteed
fn clear_current_level() {
    clean_up_debug_stage();
}
