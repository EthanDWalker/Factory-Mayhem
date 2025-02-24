mod event;

use std::sync::Arc;

use renderer::sprite::Sprite;

pub struct Core<'a> {
    event_loop: winit::event_loop::EventLoop<()>,
    window: Arc<winit::window::Window>,
    renderer: renderer::Renderer<'a>,
    sprites: renderer::sprite_list::SpriteList,
    reset_level: bool,
    win_level: bool,
    level: u32,
    pub input: systems::Input,
}

impl<'a> Core<'a> {
    pub async fn new() -> Self {
        env_logger::init();
        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        let window = Arc::new(
            winit::window::WindowBuilder::new()
                .build(&event_loop)
                .unwrap(),
        );

        window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

        let renderer = renderer::Renderer::new(Arc::clone(&window)).await;
        let input = systems::Input::new();

        let mut sprites = world::level_1::level(renderer.resolution.0);

        sprites.gui.push(Sprite::new(
            &std::path::Path::new(
                "/Users/ethanwalker/Developer/Rust/engine/renderer/res/reset_gui.png",
            ),
            (0, 200 - 16),
            0.0,
            vec![1],
            0,
        ));

        Self {
            event_loop,
            window,
            renderer,
            sprites,
            reset_level: false,
            win_level: false,
            level: 1,
            input,
        }
    }

    pub fn run(mut self) {
        self.event_loop
            .run(move |event, control_flow| {
                event::handle_event(
                    &mut self.renderer,
                    &*self.window,
                    &mut self.input,
                    &mut self.sprites,
                    &mut self.reset_level,
                    &mut self.win_level,
                    self.level,
                    &event,
                    control_flow,
                );
                if self.reset_level {
                    self.sprites = match self.level {
                        1 => world::level_1::level(self.renderer.resolution.0),
                        2 => world::level_2::level(self.renderer.resolution.0),
                        3 => world::level_3::level(self.renderer.resolution.0),
                        4 => world::level_4::level(self.renderer.resolution.0),
                        _ => world::level_1::level(self.renderer.resolution.0),
                    };
                    self.sprites.gui.push(Sprite::new(
                        &std::path::Path::new(
                            "/Users/ethanwalker/Developer/Rust/engine/renderer/res/reset_gui.png",
                        ),
                        (0, 200 - 16),
                        0.0,
                        vec![1],
                        0,
                    ));
                    self.reset_level = false;
                }

                if self.win_level {
                    self.level += 1;
                    self.sprites = match self.level {
                        1 => world::level_1::level(self.renderer.resolution.0),
                        2 => world::level_2::level(self.renderer.resolution.0),
                        3 => world::level_3::level(self.renderer.resolution.0),
                        4 => world::level_4::level(self.renderer.resolution.0),
                        _ => world::level_1::level(self.renderer.resolution.0),
                    };
                    self.reset_level = false;

                    self.sprites.gui.push(Sprite::new(
                        &std::path::Path::new(
                            "/Users/ethanwalker/Developer/Rust/engine/renderer/res/reset_gui.png",
                        ),
                        (0, 200 - 16),
                        0.0,
                        vec![1],
                        0,
                    ));
                    self.win_level = false;
                }
            })
            .expect("Failed");
    }
}
