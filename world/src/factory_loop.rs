use renderer::factory_belt::{FactoryBelt, FactoryBeltDirection};

/// width >= 4
/// height >= 3
pub fn factory_loop(
    width: u32,
    height: u32,
    speed: f32,
    top_left: (u32, u32),
    clockwise: bool,
) -> Vec<FactoryBelt> {
    let mut factory_loop: Vec<FactoryBelt> = Vec::new();
    factory_loop.push(FactoryBelt::new(
        top_left,
        2,
        FactoryBeltDirection::Right as usize,
        speed / 4.0,
    ));
    factory_loop.push(FactoryBelt::new(
        (top_left.0, top_left.1 + ((height - 3) * 16)),
        height - 3,
        FactoryBeltDirection::Up as usize,
        speed,
    ));
    factory_loop.push(FactoryBelt::new(
        (top_left.0, top_left.1 + (height - 1) * 16),
        2,
        FactoryBeltDirection::Up as usize,
        speed / 2.0,
    ));

    factory_loop.push(FactoryBelt::new(
        (top_left.0 + ((width - 1) * 16), top_left.1),
        2,
        FactoryBeltDirection::Down as usize,
        speed / 2.0,
    ));
    factory_loop.push(FactoryBelt::new(
        (top_left.0 + ((width - 1) * 16), top_left.1 + 2 * 16),
        height - 3,
        FactoryBeltDirection::Down as usize,
        speed,
    ));
    factory_loop.push(FactoryBelt::new(
        (
            top_left.0 + ((width - 1) * 16),
            top_left.1 + ((height - 1) * 16),
        ),
        2,
        FactoryBeltDirection::Left as usize,
        speed / 4.0,
    ));

    factory_loop.push(FactoryBelt::new(
        (top_left.0 + ((width - 3) * 16), top_left.1 + ((height - 1) * 16)),
        width - 3,
        FactoryBeltDirection::Left as usize,
        speed,
    ));

    factory_loop.push(FactoryBelt::new(
        (top_left.0 + ((2) * 16), top_left.1),
        width - 3,
        FactoryBeltDirection::Right as usize,
        speed,
    ));
    factory_loop
}
