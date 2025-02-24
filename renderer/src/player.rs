use crate::{pixel, sprite::Sprite};
use systems::Input;

#[derive(Debug)] 
pub struct Player {
    sprites: Vec<Sprite>,
    outline_sprites: Vec<Sprite>,
    pub dead: bool,
    pub hidden: bool,
}

impl Player {
    pub fn new() -> Self {
        let player = Sprite::new(
            &std::path::Path::new(
                "/Users/ethanwalker/Developer/Rust/engine/renderer/res/mouse.png",
            ),
            (0, 0),
            15.0,
            vec![6, 6, 6, 6, 6],
            0,
        );

        let player_outline = Sprite::new(
            &std::path::Path::new(
                "/Users/ethanwalker/Developer/Rust/engine/renderer/res/mouse_outline.png",
            ),
            (0, 0),
            15.0,
            vec![6, 6, 6, 6, 6],
            0,
        );

        Self {
            sprites: vec![player],
            outline_sprites: vec![player_outline],
            dead: false,
            hidden: true,
        }
    }

    pub fn render(&mut self, pixels: &mut Vec<Vec<pixel::Pixel>>, delta_time: f32) {
        for i in 0..self.sprites.len() {
            let sprite = &mut self.sprites[i];
            if self.dead {
                sprite.play_once(delta_time);
            } else {
                sprite.frame(delta_time);
            }
            sprite.draw(pixels);
        }
    }

    pub fn render_outline(&mut self, pixels: &mut Vec<Vec<pixel::Pixel>>, delta_time: f32) {
        for i in 0..self.outline_sprites.len() {
            let sprite = &mut self.outline_sprites[i];
            if self.dead {
                sprite.play_once(delta_time);
            } else {
                sprite.frame(delta_time);
            }
            sprite.draw(pixels);
        }
    }

    pub fn collides(&self, sprite: &Sprite) -> bool {
        for i in 0..self.sprites.len() {
            if sprite.collides(self.sprites[i].middle(), (1, 1)) {
                return true;
            }
        }
        false
    }

    pub fn collides_circle(&self, circle: &crate::circle::Circle) -> bool {
        for i in 0..self.sprites.len() {
            if circle.collides(self.sprites[i].middle()) {
                return true;
            }
        }
        false
    }

    pub fn handle_movement(&mut self, input: &Input, delta_time: f32) {
        for i in 0..self.sprites.len() {
            let sprite = &mut self.sprites[i];
            if self.dead {
                sprite.set_animation(4);
            } else {
                if ((input.right_pressed && input.left_pressed)
                    || (!input.left_pressed && !input.right_pressed))
                    && (!input.down_pressed && !input.up_pressed)
                {
                    if input.right_last_pressed {
                        sprite.set_animation(0);
                    } else {
                        sprite.set_animation(1);
                    }
                } else {
                    if input.left_pressed || input.up_pressed {
                        sprite.set_animation(3);
                    }
                    if input.right_pressed || input.down_pressed {
                        sprite.set_animation(2);
                    }
                }
                sprite.position.1 -= input.up_pressed as u32 as f32 * delta_time * 50.0;
                sprite.position.1 += input.down_pressed as u32 as f32 * delta_time * 50.0;
                sprite.position.0 -= input.left_pressed as u32 as f32 * delta_time * 50.0;
                sprite.position.0 += input.right_pressed as u32 as f32 * delta_time * 50.0;
                if sprite.position.0 < 0.0 { 
                    sprite.position.0 = 0.0
                }
                if sprite.position.1 < 0.0 { 
                    sprite.position.1 = 0.0
                }
            }
        }
        for i in 0..self.outline_sprites.len() {
            let sprite = &mut self.outline_sprites[i];
            if self.dead {
                sprite.set_animation(4);
            } else {
                if ((input.right_pressed && input.left_pressed)
                    || (!input.left_pressed && !input.right_pressed))
                    && (!input.down_pressed && !input.up_pressed)
                {
                    if input.right_last_pressed {
                        sprite.set_animation(0);
                    } else {
                        sprite.set_animation(1);
                    }
                } else {
                    if input.left_pressed || input.up_pressed {
                        sprite.set_animation(3);
                    }
                    if input.right_pressed || input.down_pressed {
                        sprite.set_animation(2);
                    }
                }
                sprite.position.1 -= input.up_pressed as u32 as f32 * delta_time * 50.0;
                sprite.position.1 += input.down_pressed as u32 as f32 * delta_time * 50.0;
                sprite.position.0 -= input.left_pressed as u32 as f32 * delta_time * 50.0;
                sprite.position.0 += input.right_pressed as u32 as f32 * delta_time * 50.0;
                if sprite.position.0 < 0.0 { 
                    sprite.position.0 = 0.0
                }
                if sprite.position.1 < 0.0 { 
                    sprite.position.1 = 0.0
                }
            }
        }
        self.hidden = false;
    }

    pub fn should_reset(&self) -> bool {
        if !self.dead {
            return false;
        }
        let mut reset = true;
        for i in 0..self.sprites.len() {
            let sprite = &self.sprites[i];
            if sprite.is_playing {
                reset = false;
            }
        }
        return reset;
    }
}
