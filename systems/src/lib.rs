mod input;

use winit::event::WindowEvent;

pub struct Input {
    pub delta_mouse_pos: (f32, f32),
    pub input: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub right_pressed: bool,
    pub left_pressed: bool,
    pub rotate_pressed: bool,
    pub reset_pressed: bool,
    pub right_last_pressed: bool,
}

impl Input {
    pub fn new() -> Self {
        Self {
            delta_mouse_pos: (0.0, 0.0),
            input: false,
            up_pressed: false,
            down_pressed: false,
            right_pressed: false,
            left_pressed: false,
            rotate_pressed: false,
            reset_pressed: false,
            right_last_pressed: true,
        }
    }

    pub fn get_input(&mut self, event: &WindowEvent) {
        input::parse_input(self, event);
    }
}
