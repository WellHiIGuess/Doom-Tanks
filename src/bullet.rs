use raylib::math::Rectangle;

static mut BULLET_SPEED: f32 = 50.0;
// static mut BULLET_SPEED: f32 = 1.0;

#[derive(Copy, Clone)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub direction: &'static str,
    pub rectangle: Rectangle,
    pub dead: bool,
    pub bullet_type: i8,
}

impl Bullet {
    pub fn new(x: f32, y: f32, direction: &'static str, bullet_type: i8) -> Self {
        Self {
            x,
            y,
            direction,
            rectangle: Rectangle::new(0.0, 0.0, 0.0,0.0),
            dead: false,
            bullet_type,
        }
    }

    pub unsafe fn update(&mut self, delta_time: f32, phase: usize) {
        if !self.dead && phase >= 10 {
            match self.direction {
                "north" => self.y -= BULLET_SPEED * (2.0 * self.bullet_type as f32) * delta_time,
                "south" => self.y += BULLET_SPEED * (2.0 * self.bullet_type as f32) * delta_time,
                "east" => self.x += BULLET_SPEED * (2.0 * self.bullet_type as f32) * delta_time,
                "west" => self.x -= BULLET_SPEED * (2.0 * self.bullet_type as f32) * delta_time,
                _ => {}
            }
        } else if !self.dead {
            match self.direction {
                "north" => self.y -= BULLET_SPEED * delta_time,
                "south" => self.y += BULLET_SPEED * delta_time,
                "east" => self.x += BULLET_SPEED * delta_time,
                "west" => self.x -= BULLET_SPEED * delta_time,
                _ => {}
            }
        }

        if phase >= 10 {
            self.rectangle = Rectangle::new(self.x - 1.0 * 7.0 + 25.0, self.y - 1.0 * 7.0 + 25.0, 7.0 * 3.0, 7.0 * 3.0);
        }
        self.rectangle = Rectangle::new(self.x * 7.0 + 25.0, self.y * 7.0 + 25.0, 7.0, 7.0);
    }
}