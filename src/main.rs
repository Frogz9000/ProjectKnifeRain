use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use rand::Rng;
mod camera;
use camera::CameraControls;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(CameraControls)
        .add_systems(Startup, setup_test_level)
        .run();
    
}

fn setup_test_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    commands.spawn(PointLight{intensity: 1000.0,..Default::default()});
    let mesh_handle = meshes.add(Cuboid::new(1.0,1.0,1.0));
    //spawn in ground layer; 50x50 meter
    for i in 0..49{
        for j in 0..=49{
            //TODO apply basic gpraphic to these colliders with PBR
            //Have each row or coulmn have a unique color to better understand map positioning
            //I want to see the coordinate plane
            //spawn in colliders 
            commands
            .spawn(Collider::cuboid(0.5, 0.5, 0.5))
            .insert(Transform::from_xyz(i as f32, 0.0, j as f32));
            //spawn visual block for each
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(materials.add(StandardMaterial{
                        base_color: Color::LinearRgba(LinearRgba { red: random_color(), green: random_color(), blue: random_color(), alpha: 1.0 }),
                        ..Default::default()
                }
            )),
            Transform::from_xyz(i as f32, 0.0, j as f32)));
        };
    }
}

fn random_color()->f32{
    let mut rng = rand::rng();
    let value: f32 = rng.random();
    return value;
}