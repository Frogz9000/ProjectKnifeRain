use crate::player_class_actions::wizard::WizardPlugin;
use crate::level_management::world_controller::WorldController;
use crate::{netcode::NetcodePlugin, spells::SpellsPlugin, ui::UIPlugin};
use bevy::window::PresentMode;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;
use player::PlayerPlugin;
mod input_map;
mod netcode;
mod player;
mod player_class_actions;
mod spells;
mod ui;
mod level_management;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::Immediate,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(WorldController)
        .add_plugins(PlayerPlugin)
        .add_plugins(NetcodePlugin)
        .add_plugins(WizardPlugin)
        .add_plugins(SpellsPlugin)
        .add_plugins(UIPlugin)
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
