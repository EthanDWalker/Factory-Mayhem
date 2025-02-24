use renderer::{
    factory_belt::{FactoryBelt, FactoryBeltDirection},
    factory_crate::{CrateType, FactoryCrate},
    player::Player,
    sprite::Sprite,
    sprite_list::SpriteList,
};

use factory_loop::factory_loop;

use crate::factory_loop;

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
    let loop_width = 6;

    let factory_crate_0 = FactoryCrate::new(
        CrateType::GunPowder,
        ((width / 2) - ((loop_width * 16) / 2), 16),
    );

    let sprites: SpriteList;
    let mut factory_belts: Vec<Sprite> = Vec::new();

    let factory_loop_0 = factory_loop(
        loop_width,
        loop_height,
        20.0,
        ((width / 2) - ((loop_width * 16) / 2), 16),
        true,
    );

    for belt in factory_loop_0 {
        for piece in belt.belt {
            factory_belts.push(piece);
        }
    }

    let factory_crate_1 = FactoryCrate::new(
        CrateType::Oil,
        ((width / 2) - ((loop_width * 16) / 2) + (16 * 2), 32 + (loop_height * 16)),
    );
    factory_belts.push(FactoryBelt::new(
        ((width / 2) - ((loop_width * 16) / 2) + (16 * 2), 16 + (loop_height * 16)),
        1,
        FactoryBeltDirection::Down as usize,
        10.0,
    ).belt[0].clone());

    sprites = SpriteList {
        factory_belts,
        player: Some(player),
        factory_crates: vec![factory_crate_0, factory_crate_1],
        background: vec![background],
        other_outline: vec![],
        spotlights: vec![],
        explosion: None,
        gui: vec![],
    };

    sprites
}
