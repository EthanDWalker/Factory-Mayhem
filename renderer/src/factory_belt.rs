use crate::sprite::Sprite;

pub enum FactoryBeltDirection {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

pub fn get_belt_direction_tuple(direction: usize, delta_time: f32) -> (f32, f32) {
    match direction {
        0 => (delta_time, 0.0),
        1 => (0.0, delta_time),
        2 => (-delta_time, 0.0),
        3 => (0.0, -delta_time),
        _ => (0.0, 0.0),
    }
}

#[derive(Debug)]
pub struct FactoryBelt {
    pub belt: Vec<Sprite>,
}

impl FactoryBelt {
    pub fn new(
        start_position: (u32, u32),
        pieces: u32,
        direction: usize,
        animation_speed: f32,
    ) -> Self {
        let mut belt: Vec<Sprite> = Vec::new();
        let img_path = std::path::Path::new(
            "/Users/ethanwalker/Developer/Rust/engine/renderer/res/factory_belt.png",
        );
        let direction_tuple = get_belt_direction_tuple(direction, 1.0);
        for i in 0..pieces as i32 {
            belt.push(Sprite::new(
                img_path,
                (
                    (direction_tuple.0 as i32 * (i * 16) + start_position.0 as i32) as u32,
                    (direction_tuple.1 as i32 * (i * 16) + start_position.1 as i32) as u32,
                ),
                animation_speed,
                vec![16, 16, 16, 16],
                direction,
            ));
        }
        Self { belt }
    }
}
