use bevy_rapier3d::{na::Point3, parry::shape::SharedShape, prelude::{Collider, RigidBody}};
use rand::Rng;
use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::{Indices, PrimitiveTopology}};

pub struct GenerateWorldPlugin;

impl Plugin for GenerateWorldPlugin {
    fn build(&self, app: &mut App){
        app
            .add_systems(Startup, setup_test_level);
    }
}

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
    let (collider_ramp,mesh_ramp) = generate_ramp(5.0, 5.0, 5.0);
    let mesh_ramp_handle = meshes.add(mesh_ramp);
    commands.spawn((
        Mesh3d(mesh_ramp_handle.clone()),
        MeshMaterial3d(materials.add(StandardMaterial{
                        base_color: Color::LinearRgba(LinearRgba { red: random_color(), green: random_color(), blue: random_color(), alpha: 1.0 }),
                        ..Default::default()
        })),
        collider_ramp,
        RigidBody::Fixed,
        Transform::from_xyz(10.0, -1.0, 10.0),
    ));
}

//generate a mesh and collider of a ramp. Apply transform after generation to rotate and move
fn generate_ramp(
    height: f32,
    depth: f32,
    width: f32,
) -> (Collider, Mesh)
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

    let verticies = vec![
        //same as points but with converted type
        [0.0,0.0,0.0],
        [width,0.0,0.0],
        [0.0,0.0,depth],
        [width,0.0,depth],
        [width,height,depth],
        [0.0,height,depth],
    ];

    let indicies: Vec<u32> = vec![
        // bottom
        0, 1, 3,
        0, 3, 2,
        // ramp slope
        2, 3, 4,
        2, 4, 5,
        // right wall
        1, 3, 4,
        // left wall
        0, 2, 5,
        // back wall
        2, 5, 4,
        2, 4, 3,
    ];
    
    let shape_generate = SharedShape::convex_hull(&points).expect("whatver burger");
    let collider = Collider::from(shape_generate.clone());
    let normals = vec![[0.0,1.0,0.0]; verticies.len()];//placeholder normal
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verticies);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.indices_mut().replace(&mut Indices::U32(indicies));
    //mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    return (collider, mesh);

    
}

fn random_color()->f32{
    let mut rng = rand::rng();
    let value: f32 = rng.random();
    return value;
}