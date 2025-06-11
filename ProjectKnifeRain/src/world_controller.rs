use bevy::ecs::{component::Component, entity::Entity, query::With, system::{Commands, Query}};

fn cleanup_with_component<T: Component>(
    mut command: Commands,
    q: Query<Entity, With<T>>,
){
    for e in q.iter(){
        command.entity(e).despawn();
    }
}
