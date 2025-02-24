
fn main() {
    let game_core = pollster::block_on(core::Core::new());
    game_core.run();
}
