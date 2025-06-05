use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::CursorGrabMode};
use bevy_rapier3d::{prelude::*};
use player::PlayerPlugin;

use crate::{generate_world::GenerateWorldPlugin, netcode::NetcodePlugin};

mod player;
mod netcode;
mod generate_world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(( PlayerPlugin, GenerateWorldPlugin))
        // .add_plugins(NetcodePlugin)
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