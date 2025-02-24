use crate::factory_loop::factory_loop;
use renderer::{
    circle::Circle,
    factory_belt::{FactoryBelt, FactoryBeltDirection},
    factory_crate::{CrateType, FactoryCrate},
    player::Player,
    sprite::Sprite,
    sprite_list::SpriteList,
};

pub fn level(width: u32) -> SpriteList {
    let player = Player::new();

    let background = Sprite::new(
        &std::path::Path::new(
            "/Users/ethanwalker/Developer/Rust/engine/renderer/res/background.png",
        ),
        (0, 0),
        0.0,
        vec![1],
        0,
    );

    let loop_height = 4;
    let loop_width = 4;

    let factory_crate_0 = FactoryCrate::new(CrateType::GunPowder, (0, 0));
    let factory_crate_1 = FactoryCrate::new(CrateType::Oil, (240, 176));

    let sprites: SpriteList;
    let mut factory_belts: Vec<Sprite> = Vec::new();

    for j in 0..3 {
        for i in 0..4 {
            let factory_loop_0 = factory_loop(
                loop_width,
                loop_height,
                30.0,
                ((16 * 4 * i), (16 * 4 * j)),
                true,
            );

            for belt in factory_loop_0 {
                for piece in belt.belt {
                    factory_belts.push(piece);
                }
            }
        }
    }

    sprites = SpriteList {
        factory_belts,
        player: Some(player),
        factory_crates: vec![factory_crate_0, factory_crate_1],
        background: vec![background],
        other_outline: vec![],
        spotlights: vec![
            Circle::new(
                (33.0, 33.0),
                32,
                Vec::from([(width - 33, 33, 40.0), (0, 33, 40.0)]),
            ),
            Circle::new(
                (width as f32 / 2.0, 100.0),
                32,
                Vec::from([(width - 33, 100, 40.0), (33, 100, 40.0)]),
            ),
            Circle::new(
                (0.0, 166.0),
                32,
                Vec::from([(width - 33, 166, 40.0), (33, 166, 40.0)]),
            ),
        ],
        explosion: None,
        gui: vec![],
    };

    sprites
}
