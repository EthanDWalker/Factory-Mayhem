use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey::Code},
};

use crate::Input;

pub fn parse_input(input: &mut Input, event: &WindowEvent) {
    match event {
        WindowEvent::KeyboardInput {
            device_id: _,
            event:
                KeyEvent {
                    state,
                    physical_key: Code(keycode),
                    ..
                },
            is_synthetic: _,
        } => {
            let is_pressed = *state == ElementState::Pressed;
            match keycode {
                KeyCode::KeyA => {
                    input.left_pressed = is_pressed;
                    input.right_last_pressed = false;
                }
                KeyCode::KeyD => {
                    input.right_pressed = is_pressed;
                    input.right_last_pressed = true;
                }
                KeyCode::KeyW => {
                    input.up_pressed = is_pressed;
                }
                KeyCode::KeyS => {
                    input.down_pressed = is_pressed;
                }
                KeyCode::KeyR => {
                    input.rotate_pressed = is_pressed;
                }
                KeyCode::KeyM => {
                    input.reset_pressed = is_pressed;
                }
                _ => {}
            }
        }
        _ => {}
    }
}
