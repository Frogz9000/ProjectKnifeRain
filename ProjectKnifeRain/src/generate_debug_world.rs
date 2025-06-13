use bevy_rapier3d::{na::Point3, parry::shape::SharedShape, prelude::{Collider, RigidBody}};
use rand::Rng;
use bevy::prelude::*;

pub struct GenerateDebugWorldPlugin;

impl Plugin for GenerateDebugWorldPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, setup_test_level);
    }
}

#[derive(Component)]
pub struct DebugStageCleanup;

fn setup_test_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
){
    commands.spawn(PointLight{intensity: 1000.0,..Default::default()});
    let mesh_handle = meshes.add(Cuboid::new(1.0,1.0,1.0));
    //spawn in ground layer; 50x50 meter
    for i in 0..100{
        for j in 0..=99{
            commands.spawn((
                Mesh3d(mesh_handle.clone()),
                MeshMaterial3d(materials.add(StandardMaterial{
                        base_color: Color::LinearRgba(LinearRgba { red: random_color(), green: random_color(), blue: random_color(), alpha: 1.0 }),
                        ..Default::default()
                })),
                Transform::from_xyz(i as f32, -1.0, j as f32),
                ));
        };
    }
    commands.spawn((
        Collider::cuboid(50.0, 0.5, 50.0),
        RigidBody::Fixed,
        Transform::from_xyz(49.5, -1.0, 49.5),
    ));
    commands.spawn((
        generate_ramp(5.0, 5.0, 5.0),
        RigidBody::Fixed,
        Transform::from_xyz(10.0, -1.0, 10.0),
    ));
}

//generate a mesh and collider of a ramp. Apply transform after generation to rotate and move
fn generate_ramp(
    height: f32,
    depth: f32,
    width: f32,
) -> Collider
{
    let points =[
        //base rectangle
        Point3::new(0.0,0.0,0.0),//bottom left front corner
        Point3::new(width,0.0,0.0),//bottom right front corner
        Point3::new(0.0,0.0,depth),//bottom left back corner
        Point3::new(width,0.0,depth),//bottom right back corner
        //top of ramp triangle
        Point3::new(width,height,depth),//top right
        Point3::new(0.0,height,depth),//top left
    ];
    
    let shape_generate = SharedShape::convex_hull(&points).expect("whatver burger");
    let collider = Collider::from(shape_generate.clone());
    return collider;

    
}

fn random_color()->f32{
    let mut rng = rand::rng();
    let value: f32 = rng.random();
    return value;
}
