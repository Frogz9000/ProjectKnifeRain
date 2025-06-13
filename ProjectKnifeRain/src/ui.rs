use bevy::{
    color::palettes::css::GOLD,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_text_ui);
        app.add_systems(Update, update_ui_text);
    }
}

#[derive(Component)]
struct FrameText;

#[derive(Component)]
struct PauseMenu;

fn setup_text_ui(mut command: Commands, _asset_server: Res<AssetServer>) {
    command.spawn((Text::new("FPS: "),)).with_child((
        TextSpan::default(),
        TextColor(GOLD.into()),
        FrameText,
    ));
}

fn update_ui_text(
    diagnostics: Res<DiagnosticsStore>,
    mut frame_query: Query<&mut TextSpan, With<FrameText>>,
) {
    for mut span in &mut frame_query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(frame_num) = fps.smoothed() {
                **span = format!("{frame_num:.2}");
            }
        }
    }
}

fn setup_pause_menu(mut command: Commands) {}
//have esc key always be open/close ie no key map
fn open_pause_menu(mut command: Commands, input: Res<ButtonInput<KeyCode>>) {}
