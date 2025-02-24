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

    let loop_height = 3;
    let loop_width = 4;

    let factory_crate_0 = FactoryCrate::new(CrateType::GunPowder, (16, 16));
    let factory_crate_1 = FactoryCrate::new(CrateType::GunPowder, (16 * 3, 16 * 5));

    let factory_crate_2 = FactoryCrate::new(
        CrateType::Oil,
        (
            (width / 2) - ((loop_width * 16) / 2),
            32 + (loop_height * 16),
        ),
    );

    let sprites: SpriteList;
    let mut factory_belts: Vec<Sprite> = Vec::new();

    let factory_loop_0 = factory_loop(loop_width, loop_height, 20.0, (16, 16), true);

    let factory_loop_1 = factory_loop(loop_width, loop_height, 20.0, (16, 16 * 5), true);

    for belt in factory_loop_0 {
        for piece in belt.belt {
            factory_belts.push(piece);
        }
    }

    for belt in factory_loop_1 {
        for piece in belt.belt {
            factory_belts.push(piece);
        }
    }

    let factory_belt = FactoryBelt::new(
        (17 + (loop_width * 16), 32),
        2,
        FactoryBeltDirection::Right as usize,
        20.0,
    );

    for i in factory_belt.belt {
        factory_belts.push(i);
    }

    let factory_belt = FactoryBelt::new(
        (17 + (loop_width * 16), 48 + ((loop_height - 1) * 16)),
        2,
        FactoryBeltDirection::Right as usize,
        20.0,
    );

    for i in factory_belt.belt {
        factory_belts.push(i);
    }

    let factory_belt = FactoryBelt::new(
        (
            15 + (16 * 2) + (loop_width * 16),
            48 + ((loop_height - 1) * 16),
        ),
        2,
        FactoryBeltDirection::Up as usize,
        10.0,
    );

    for i in factory_belt.belt {
        factory_belts.push(i);
    }

    let factory_loop_2 = factory_loop(loop_width, loop_height, 20.0, (16 * 9, 32), true);

    let factory_crate_2 = FactoryCrate::new(CrateType::GunPowder, (16 * 9, 32));
    let factory_crate_3 = FactoryCrate::new(CrateType::Oil, (16 * 13, 32));

    for belt in factory_loop_2 {
        for piece in belt.belt {
            factory_belts.push(piece);
        }
    }

    sprites = SpriteList {
        factory_belts,
        player: Some(player),
        factory_crates: vec![factory_crate_0, factory_crate_1, factory_crate_2, factory_crate_3],
        background: vec![background],
        other_outline: vec![],
        spotlights: vec![
            Circle::new(
                (100.0, 100.0),
                100,
                Vec::from([(width / 2 - 16, 100, 0.0)]),
            ),
        ],
        explosion: None,
        gui: vec![],
    };

    sprites
}
