use bevy::esc::component::Component

fn cleanup_with_component<T: Component>(
    mut command: Commands,
    q: Query<Entity, With<T>>,
){
    for e in q.iter(){
        command.entity(e).despawn()
}
