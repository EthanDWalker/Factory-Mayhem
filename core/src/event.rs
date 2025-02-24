pub fn handle_event(
    renderer: &mut renderer::Renderer,
    window: &winit::window::Window,
    input: &mut systems::Input,
    sprites: &mut renderer::sprite_list::SpriteList,
    reset_level: &mut bool,
    win_level: &mut bool,
    level: u32,
    event: &winit::event::Event<()>,
    control_flow: &winit::event_loop::EventLoopWindowTarget<()>,
) {
    match event {
        winit::event::Event::WindowEvent { event, .. } => match event {
            winit::event::WindowEvent::Resized(physical_size) => {
                renderer.resize(*physical_size);
            }
            winit::event::WindowEvent::RedrawRequested => {
                window.request_redraw();

                let now = std::time::Instant::now();
                let delta_time = now
                    .duration_since(renderer.last_render_time)
                    .as_secs_f32();

                renderer
                    .update(input, sprites, reset_level, win_level, level, delta_time);
                renderer.last_render_time = now;
                renderer.render();
            }
            winit::event::WindowEvent::KeyboardInput { .. } => {
                input.get_input(&event);
            }
            winit::event::WindowEvent::CloseRequested => {
                control_flow.exit();
            }
            _ => {}
        },
        _ => {}
    }
}
