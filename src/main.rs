use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, setup_test_level)
        .add_systems(Startup, setup_graphics)
        .run();
}

fn setup_test_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    commands.spawn(PointLight{intensity: 1000.0,..Default::default()});
    let mesh_handle = meshes.add(Cuboid::new(1.0,1.0,1.0));
    let material_handle = materials.add(StandardMaterial{
        base_color: Color::LinearRgba(LinearRgba { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0 }),..Default::default()
    });
    //spawn in ground layer; 50x50 meter
    for i in 0..50{
        for j in 0..50{
            //TODO apply basic gpraphic to these colliders with PBR
            //Have each row or coulmn have a unique color to better understand map positioning
            //I want to see the coordinate plane
            //spawn in colliders 
            commands
            .spawn(Collider::cuboid(0.5, 0.5, 0.5))
            .insert(Transform::from_xyz(i as f32, j as f32, 0.0));
            //spawn visual block for each
            
        }
    }
}


fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(Transform::from_xyz(0.0, -2.0, 0.0));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 4.0, 0.0));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: {}", transform.translation.y);
    }
}