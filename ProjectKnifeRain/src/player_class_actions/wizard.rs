use crate::player::*;
use crate::spells::*;
use bevy::prelude::*;

pub struct WizardPlugin;
impl Plugin for WizardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shoot_fireball_forward,));
    }
}

fn shoot_fireball_forward(
    mut spell_event: EventWriter<CastSpellEvent>,
    spawner_query: Query<&GlobalTransform, With<SpawnerMuzzle>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    let Some(spawn_info) = get_spawner_look_dir_and_pos(spawner_query) else {
        return;
    };
    let origin = spawn_info.position;
    let direction = spawn_info.direction;
    if input.just_pressed(MouseButton::Left) {
        spell_event.write(CastSpellEvent {
            spell_type: SpellTypes::Fireball,
            origin: origin,
            direction: direction,
        });
    }
}
