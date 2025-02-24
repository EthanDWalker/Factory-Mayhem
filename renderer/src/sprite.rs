use image::Rgba;

use crate::pixel;

#[derive(Clone, Debug)]
pub struct Sprite {
    pub dimensions: (u32, u32),
    pub position: (f32, f32),
    image: image::RgbaImage,
    current_frame: f32,
    pub animation_speed: f32,
    pub is_playing: bool,
    animation_lens: Vec<u32>,
    pub current_animation: usize,
}

impl Sprite {
    pub fn new(
        img_path: &std::path::Path,
        position: (u32, u32),
        animation_speed: f32,
        animation_lens: Vec<u32>,
        current_animation: usize,
    ) -> Self {
        let image = image::ImageReader::open(img_path)
            .expect("Failed to open image")
            .decode()
            .expect("Failed to decode image")
            .into_rgba8();

        Self {
            dimensions: (
                image.dimensions().0 / animation_lens.iter().max().unwrap(),
                image.dimensions().1 / animation_lens.len() as u32,
            ),
            position: (position.0 as f32, position.1 as f32),
            image,
            current_frame: 0.0,
            animation_speed,
            animation_lens,
            current_animation,
            is_playing: true,
        }
    }

    pub fn draw(&self, pixels: &mut Vec<Vec<pixel::Pixel>>) {
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                let index_x = (x + self.position.0 as u32) as usize;
                let index_y = (y + self.position.1 as u32) as usize;
                if index_x < pixels.len() && index_y < pixels[x as usize].len() {
                    let pixel = self.get_pixel_rgb(x, y);
                    if pixel != None {
                        pixels[index_x as usize][index_y as usize].color = pixel.unwrap();
                    }
                }
            }
        }
    }

    fn get_pixel_rgb(&self, x: u32, y: u32) -> Option<[f32; 3]> {
        let pixel = self
            .image
            .get_pixel_checked(
                x + (self.current_frame as u32 * self.dimensions.0),
                y + (self.current_animation as u32 * self.dimensions.1),
            )
            .unwrap_or_else(|| return &Rgba([0, 0, 0, 0]));
        if pixel[3] == 0 {
            return None;
        }
        Some([
            pixel[0] as f32 / 255.0,
            pixel[1] as f32 / 255.0,
            pixel[2] as f32 / 255.0,
        ])
    }

    pub fn frame(&mut self, delta_time: f32) {
        if !self.is_playing {
            return;
        }
        self.current_frame += delta_time * self.animation_speed;
        if self.current_frame > self.animation_lens[self.current_animation] as f32 {
            self.current_frame = 0.0;
        }
    }

    pub fn play_once(&mut self, delta_time: f32) {
        if !self.is_playing {
            return;
        }
        self.current_frame += delta_time * self.animation_speed;
        if self.current_frame > self.animation_lens[self.current_animation] as f32 {
            self.is_playing = false;
        }
    }

    pub fn set_animation(&mut self, index: usize) {
        if self.current_animation == index {
            return;
        };
        self.is_playing = true;
        self.current_frame = 0.0;
        self.current_animation = if index < self.animation_lens.len() {
            index
        } else {
            0
        };
    }

    pub fn reset_animation(&mut self) {
        self.current_frame = 0.0;
    }

    pub fn middle(&self) -> (f32, f32) {
        (
            (self.position.0 * 2.0 + self.dimensions.0 as f32) / 2.0,
            (self.position.1 * 2.0 + self.dimensions.1 as f32) / 2.0,
        )
    }

    pub fn collides(&self, position: (f32, f32), dimensions: (u32, u32)) -> bool {
        if (self.position.0) < dimensions.0 as f32 + position.0
            && (self.position.0) + self.dimensions.0 as f32 > position.0
            && (self.position.1) < dimensions.1 as f32 + position.1
            && (self.position.1) + self.dimensions.1 as f32 > position.1
        {
            return true;
        }
        false
    }
}
