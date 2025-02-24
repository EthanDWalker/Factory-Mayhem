use crate::{
    pixel, player,
    sprite::{self, Sprite},
    sprite_list::SpriteList,
    Renderer,
};

pub fn render(renderer: &mut Renderer) -> Result<(), wgpu::SurfaceError> {
    let output = renderer.surface.get_current_texture()?;
    let view = output
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());

    let mut encoder = renderer
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        render_pass.set_pipeline(&renderer.pipeline.pipeline);
        render_pass.set_bind_group(0, &renderer.resolution_bind_group.bind_group, &[]);
        render_pass.set_vertex_buffer(0, renderer.pipeline.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, renderer.pipeline.pixel_buffer.slice(..));
        render_pass.set_index_buffer(
            renderer.pipeline.index_buffer.slice(..),
            wgpu::IndexFormat::Uint16,
        );
        render_pass.draw_indexed(
            0..pixel::PIXEL_MODEL_VIEW_INDICES.len() as u32,
            0,
            0..(renderer.resolution.0 * renderer.resolution.1) as _,
        );
    }

    renderer.queue.submit(std::iter::once(encoder.finish()));
    output.present();

    Ok(())
}

pub fn update(
    renderer: &mut Renderer,
    input: &mut systems::Input,
    sprites: &mut SpriteList,
    reset_level: &mut bool,
    win_level: &mut bool,
    level: u32,
    delta_time: f32,
) {
    let player: &mut player::Player = if let Some(player) = &mut sprites.player {
        player
    } else {
        // handle menu
        return;
    };

    if player.should_reset() {
        *reset_level = true;
    };
    if input.reset_pressed {
        *reset_level = true;
    }

    for i in 0..sprites.background.len() {
        let sprite = &mut sprites.background[i];
        sprite.frame(delta_time);
        sprite.draw(&mut renderer.pipeline.pixels);
    }

    player.handle_movement(input, delta_time);

    for i in 0..sprites.spotlights.len() {
        let spotlight = &mut sprites.spotlights[i];
        spotlight.path_move(delta_time);
    }

    for x in 0..renderer.pipeline.pixels.len() {
        for y in 0..renderer.pipeline.pixels[x].len() {
            let pixel = &mut renderer.pipeline.pixels[x][y];
            pixel.lighting = 0.2;
        }
    }

    for x in 0..renderer.pipeline.pixels.len() {
        for y in 0..renderer.pipeline.pixels[x].len() {
            let pixel = &mut renderer.pipeline.pixels[x][y];
            if sprites.spotlights.len() == 0 {
                pixel.lighting = 1.0
            }
            for i in 0..sprites.spotlights.len() {
                let spotlight = &sprites.spotlights[i];
                if spotlight.collides((pixel.position[0] as f32, pixel.position[1] as f32)) {
                    pixel.lighting = 1.0;
                }
            }
        }
    }
    player.render(&mut renderer.pipeline.pixels, delta_time);

    for i in 0..sprites.factory_belts.len() {
        let sprite = &mut sprites.factory_belts[i];
        if input.rotate_pressed {
            sprite.reset_animation();
        }
        if player.collides(sprite) {
            player.hidden = true;
            if input.rotate_pressed {
                sprite.set_animation(sprite.current_animation + 1);
                input.rotate_pressed = false;
            } else {
                sprites.other_outline.push(
                            Sprite::new(
                                std::path::Path::new("/Users/ethanwalker/Developer/Rust/engine/renderer/res/factory_belt_highlight.png"),
                                (sprite.position.0 as u32, sprite.position.1 as u32),
                                0.0, vec![1], 0
                            )
                        );
            }
        }
        sprite.frame(delta_time);
        sprite.draw(&mut renderer.pipeline.pixels);
    }

    for i in 0..sprites.factory_crates.len() {
        let factory_crate = &mut sprites.factory_crates[i];
        let mut direction_final = (0.0, 0.0);
        for i in 0..sprites.factory_belts.len() {
            let belt = &sprites.factory_belts[i];
            if belt.collides(
                factory_crate.crate_sprite.position,
                factory_crate.crate_sprite.dimensions,
            ) {
                let direction = crate::factory_belt::get_belt_direction_tuple(
                    belt.current_animation,
                    delta_time,
                );
                direction_final.0 += direction.0 * belt.animation_speed;
                direction_final.1 += direction.1 * belt.animation_speed;
            }
        }
        factory_crate.move_direction(direction_final);
        if player.collides(&factory_crate.crate_sprite) {
            player.hidden = true;
        }
        factory_crate.draw(&mut renderer.pipeline.pixels);
    }

    player.render_outline(&mut renderer.pipeline.pixels, delta_time);
    for i in 0..sprites.spotlights.len() {
        let spotlight = &mut sprites.spotlights[i];
        if player.collides_circle(spotlight) && !player.hidden {
            player.dead = true;
        }
    }

    for i in 0..sprites.other_outline.len() {
        let sprite = &mut sprites.other_outline[i];
        sprite.frame(delta_time);
        let pixels = &mut renderer.pipeline.pixels;
        for x in 0..sprite.dimensions.0 {
            for y in 0..sprite.dimensions.1 {
                let index_x = (x + sprite.position.0 as u32) as usize;
                let index_y = (y + sprite.position.1 as u32) as usize;
                if index_x < pixels.len() && index_y < pixels[x as usize].len() {
                    pixels[index_x as usize][index_y as usize].lighting = 1.0;
                }
            }
        }
        sprite.draw(&mut renderer.pipeline.pixels);
    }

    for i in 0..sprites.gui.len() {
        let sprite = &mut sprites.gui[i];
        sprite.frame(delta_time);
        let pixels = &mut renderer.pipeline.pixels;
        sprite.draw(pixels);
        for x in 0..sprite.dimensions.0 {
            for y in 0..sprite.dimensions.1 {
                let index_x = (x + sprite.position.0 as u32) as usize;
                let index_y = (y + sprite.position.1 as u32) as usize;
                if index_x < pixels.len() && index_y < pixels[x as usize].len() {
                    pixels[index_x as usize][index_y as usize].lighting = 1.0;
                }
            }
        }
    }

    let mut explosion_pos: (f32, f32) = (-10.0, -10.0);
    let won = match level {
        1 | 2 | 3 | 5 => {
            if sprites.factory_crates.len() == 2 {
                let crate_0 = &sprites.factory_crates[0];
                let crate_1 = &sprites.factory_crates[1];
                if crate_0.crate_sprite.collides(
                    crate_1.crate_sprite.position,
                    crate_1.crate_sprite.dimensions,
                ) {
                    explosion_pos = crate_1.crate_sprite.middle();
                    true
                } else {
                    false
                }
            } else {
                *reset_level = true;
                false
            }
        }
        4 => {
            let mut ret = false;
            for i in &sprites.factory_crates {
                for j in &sprites.factory_crates {
                    if i.ty != j.ty {
                        if i.crate_sprite
                            .collides(j.crate_sprite.position, j.crate_sprite.dimensions)
                        {
                            explosion_pos = j.crate_sprite.middle();
                            ret = true
                        }
                    }
                }
            }
            ret
        }
        _ => false,
    };

    sprites.other_outline.clear();

    if won {
        if let Some(sprite) = &mut sprites.explosion {
            println!("{:?}", sprite.dimensions);
            sprite.play_once(delta_time);
            sprite.draw(&mut renderer.pipeline.pixels);
            if !sprite.is_playing {
                *win_level = true
            }
        } else {
            println!("crates");
            sprites.explosion = Some(Sprite::new(
                &std::path::Path::new(
                    "/Users/ethanwalker/Developer/Rust/engine/renderer/res/explosion.png",
                ),
                (0, 0),
                10.0,
                vec![12],
                0,
            ));
            if let Some(sprite) = &mut sprites.explosion {
                sprite.position = (
                    explosion_pos.0 - (sprite.dimensions.0 as f32 / 2.0),
                    explosion_pos.1 - (sprite.dimensions.1 as f32 / 2.0),
                )
            }
        }
    }

    renderer.queue.write_buffer(
        &renderer.pipeline.pixel_buffer,
        0,
        bytemuck::cast_slice(&renderer.pipeline.pixels_raw()),
    );
}
