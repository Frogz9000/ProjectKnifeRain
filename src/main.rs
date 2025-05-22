use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::CursorGrabMode};
use bevy_rapier3d::{prelude::*};
use rand::Rng;
mod player;
use player::PlayerPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_test_level)
        .add_systems(Update, grab_mouse)
        .run();
}

fn grab_mouse(
    mut window: Single<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        window.cursor_options.visible = false;
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
    }
    if key.just_pressed(KeyCode::Escape) {
        window.cursor_options.visible = true;
        window.cursor_options.grab_mode = CursorGrabMode::None;
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
    commands.spawn((
        Mesh3d(mesh_handle.clone()),
        MeshMaterial3d(materials.add(StandardMaterial{
                        base_color: Color::LinearRgba(LinearRgba { red: random_color(), green: random_color(), blue: random_color(), alpha: 1.0 }),
                        ..Default::default()
        })),
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
        Vec3::new(0.0,0.0,0.0),//bottom left front corner
        Vec3::new(width,0.0,0.0),//bottom right front corner
        Vec3::new(0.0,0.0,depth),//bottom left back corner
        Vec3::new(width,0.0,depth),//bottom right back corner
        //top of ramp triangle
        Vec3::new(width,height,depth),//top right
        Vec3::new(0.0,height,depth),//top left
    ];
    
    return Collider::convex_hull(&points).expect("Points preset to guarentee convex");
    
}

fn random_color()->f32{
    let mut rng = rand::rng();
    let value: f32 = rng.random();
    return value;
}