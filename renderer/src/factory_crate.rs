use crate::{pixel, sprite::Sprite};

#[derive(PartialEq)]
pub enum CrateType {
    GunPowder,
    Oil,
}

pub struct FactoryCrate {
    pub crate_sprite: Sprite,
    pub icon_sprite: Sprite,
    pub ty: CrateType,
}

impl FactoryCrate {
    pub fn new(ty: CrateType, position: (u32, u32)) -> Self {
        let crate_sprite = Sprite::new(
            &std::path::Path::new(
                "/Users/ethanwalker/Developer/Rust/engine/renderer/res/factory_crate.png",
            ),
            position,
            0.0,
            vec![1],
            0,
        );

        let icon_sprite = Sprite::new(
            &std::path::Path::new(&format!(
                "/Users/ethanwalker/Developer/Rust/engine/renderer/res/crate_icons/{}.png",
                match ty {
                    CrateType::GunPowder => "gunpowder",
                    CrateType::Oil => "oil",
                }
            )),
            (position.0 + 2, position.1 + 2),
            0.0,
            vec![1],
            0,
        );

        Self {
            crate_sprite,
            icon_sprite,
            ty,
        }
    }

    pub fn collides(&self, position: (f32, f32)) -> bool {
        self.crate_sprite.collides(position, (1, 1))
    }

    pub fn move_direction(&mut self, direction: (f32, f32)) {
        self.crate_sprite.position = (
            self.crate_sprite.position.0 + direction.0,
            self.crate_sprite.position.1 + direction.1,
        );
        if self.crate_sprite.position.0 > 0.0 {
            self.icon_sprite.position.0 = self.icon_sprite.position.0 + direction.0;
        } else {
            self.crate_sprite.position.0 = 0.0
        }
        if self.crate_sprite.position.1 > 0.0 {
            self.icon_sprite.position.1 = self.icon_sprite.position.1 + direction.1;
        } else {
            self.crate_sprite.position.1 = 0.0
        }
    }

    pub fn draw(&mut self, pixels: &mut Vec<Vec<pixel::Pixel>>) {
        self.crate_sprite.draw(pixels);
        self.icon_sprite.draw(pixels);
    }
}
