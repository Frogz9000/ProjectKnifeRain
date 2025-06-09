use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct UIPlugin;
impl Plugin for UIPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Startup, setup_text_ui);
        app.add_systems(Update,(update_ui_text));
    }
}

#[derive(Component)]
struct FrameText;

fn setup_text_ui(
    mut command: Commands,
    asset_server: Res<AssetServer>,
){

}

fn update_ui_text(

){
    
}