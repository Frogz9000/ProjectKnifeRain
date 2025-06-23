use crate::player::SpawnInfo;
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
        app.add_event::<SpawnLevelEvent>();
        app.add_systems(Startup, start_world_manager);
    }
}

#[derive(Event)]
pub struct SpawnLevelEvent {}
#[derive(Event)]
pub struct CleanupLevelEvent {}

fn start_world_manager(
    mut commands: Commands,
) {
    //this will just set appstates and hold enums for levels to use
}
