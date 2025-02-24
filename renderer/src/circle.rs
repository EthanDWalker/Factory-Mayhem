use cgmath::InnerSpace;

pub struct Circle {
    pub center: (f32, f32),
    radius: u32,
    path: Vec<(u32, u32, f32)>,
    next_path: usize,
}

impl Circle {
    pub fn new(center: (f32, f32), radius: u32, path: Vec<(u32, u32, f32)>) -> Self {
        Self {
            center,
            radius,
            path,
            next_path: 0,
        }
    }

    pub fn collides(&self, position: (f32, f32)) -> bool {
        ((position.0 as i32 - self.center.0 as i32) * (position.0 as i32 - self.center.0 as i32))
            + ((position.1 as i32 - self.center.1 as i32)
                * (position.1 as i32 - self.center.1 as i32))
            <= (self.radius * self.radius) as i32
    }

    pub fn path_move(&mut self, delta_time: f32) {
        let direction = cgmath::Vector2::new(
            self.path[self.next_path].0 as f32 - self.center.0,
            self.path[self.next_path].1 as f32 - self.center.1,
        )
        .normalize()
            * delta_time
            * self.path[self.next_path].2;

        self.center = (direction.x + self.center.0, direction.y + self.center.1);

        if self.center.0.round() == self.path[self.next_path].0 as f32
            && self.center.1.round() == self.path[self.next_path].1 as f32
        {
            self.next_path = if self.next_path + 1 == self.path.len() {
                0
            } else {
                self.next_path + 1
            }
        }
    }
}
