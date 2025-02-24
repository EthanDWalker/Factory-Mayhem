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

    let loop_height = 10;
    let loop_width = 10;

    let factory_crate_0 = FactoryCrate::new(
        CrateType::GunPowder,
        ((width-16)/2, 200 / 2),
    );
    let factory_crate_1 = FactoryCrate::new(
        CrateType::Oil,
        (
            (width / 2) + (((loop_width - 2) * 16) / 2),
            16 + ((loop_height - 1) * 16),
        ),
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

    let factory_belt_0 = FactoryBelt::new(
        (
            (width / 2) - (((loop_width - 2) * 16) / 2) + 1,
            16 * 6 as u32,
        ),
        3,
        FactoryBeltDirection::Right as usize,
        10.0,
    );

    let factory_belt_1 = FactoryBelt::new(
        (
            (width / 2) + (((loop_width - 4) * 16) / 2) - 1,
            16 * 6 as u32,
        ),
        3,
        FactoryBeltDirection::Left as usize,
        10.0,
    );

    let factory_belt_2 = FactoryBelt::new(
        ((width - 32) / 2, 32),
        4,
        FactoryBeltDirection::Down as usize,
        10.0,
    );

    let factory_belt_3 = FactoryBelt::new(
        ((width-32) / 2, 16 * 9),
        3,
        FactoryBeltDirection::Up as usize,
        10.0,
    );

    for piece in factory_belt_0.belt {
        factory_belts.push(piece);
    }

    for piece in factory_belt_1.belt {
        factory_belts.push(piece);
    }

    for piece in factory_belt_2.belt {
        factory_belts.push(piece);
    }

    for piece in factory_belt_3.belt {
        factory_belts.push(piece);
    }

    sprites = SpriteList {
        factory_belts,
        player: Some(player),
        factory_crates: vec![factory_crate_0, factory_crate_1],
        background: vec![background],
        other_outline: vec![],
        spotlights: vec![Circle::new(
            (width as f32 / 2.0, 100.0),
            90,
            Vec::from([(0, 0, 0.0)]),
        )],
        explosion: None,
        gui: vec![],
    };

    sprites
}
