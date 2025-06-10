use bevy::prelude::*;

pub struct InputMap {
    //movement keys
    pub key_forward: KeyCode,
    pub key_backward: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_sprint: KeyCode,
    //player actions
    pub key_action1: MouseButton,
    //add more
    pub key_reset_debug_stage: KeyCode,
}

impl Default for InputMap {
    fn default() -> Self {
        Self {
            key_forward: KeyCode::KeyW,
            key_backward: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_sprint: KeyCode::ShiftLeft,
            key_action1: MouseButton::Left,
            key_reset_debug_stage: KeyCode::KeyR,
        }
    }
}
