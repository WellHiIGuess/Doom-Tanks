use raylib::color::Color;
use raylib::drawing::RaylibDrawHandle;
use raylib::prelude::Rectangle;
use rand::Rng;
use crate::Player;

#[derive(Copy, Clone)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub rectangle: Rectangle,
    pub enemy_type: i8,
    pub health: i8,
    pub dead: bool,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        let rng = rand::thread_rng().gen_range(0..3);

        if rng == 0 {
            return Self { x, y, speed: 5.0, rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0), enemy_type: rng, health: 1, dead: false, };
        } else if rng == 1 {
            return Self { x, y, speed: 2.5, rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0), enemy_type: rng, health: 2, dead: false, };
        } else if rng == 2 {
            return Self { x, y, speed: 6.0, rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0), enemy_type: rng, health: 1, dead: false, };
        }


        return Self { x, y, speed: 5.0, rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0), enemy_type: rng, health: 1, dead: false, };
    }

    pub fn display(&mut self, screen: &mut [[Color; 64]; 64]) {
        let sprite_normal = [[Color::new(110, 29, 29, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(110, 29, 29, 255)], [Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(229, 204, 64, 255), Color::new(229, 204, 64, 255), Color::new(118, 66, 138, 255)], [Color::new(215, 123, 186, 255), Color::new(251, 242, 54, 255), Color::new(166, 48, 30, 255), Color::new(166, 48, 30, 255), Color::new(251, 242, 54, 255)], [Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(229, 204, 64, 255), Color::new(229, 204, 64, 255), Color::new(118, 66, 138, 255)], [Color::new(110, 29, 29, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(110, 29, 29, 255)]];
        let sprite_big = [[Color::new(110, 29, 29, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255)], [Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255)], [Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(217, 87, 99, 255), Color::new(229, 204, 64, 255), Color::new(229, 204, 64, 255), Color::new(217, 87, 99, 255), Color::new(118, 66, 138, 255)], [Color::new(110, 29, 29, 255), Color::new(215, 123, 186, 255), Color::new(251, 242, 54, 255), Color::new(172, 50, 50, 255), Color::new(172, 50, 50, 255), Color::new(251, 242, 54, 255), Color::new(215, 123, 186, 255)], [Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(217, 87, 99, 255), Color::new(229, 204, 64, 255), Color::new(229, 204, 64, 255), Color::new(217, 87, 99, 255), Color::new(118, 66, 138, 255)], [Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255)], [Color::new(110, 29, 29, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255)]];
        let sprite_speed = [[Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(106, 190, 48, 255), Color::new(251, 242, 54, 255), Color::new(106, 190, 48, 255), Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255)], [Color::new(153, 229, 80, 255), Color::new(251, 242, 54, 255), Color::new(172, 50, 50, 255), Color::new(251, 242, 54, 255), Color::new(106, 190, 48, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(106, 190, 48, 255), Color::new(251, 242, 54, 255), Color::new(106, 190, 48, 255), Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)]];

        if !self.dead && self.enemy_type == 0 {
            for x in 0..5 {
                for y in 0..5 {
                    if x as f32 + self.x > -1.0 && x as f32 + self.x < 64.0 && y as f32 + self.y > -1.0 && y as f32 + self.y < 64.0 {
                        screen[(x as f32 + self.x) as usize][(y as f32 + self.y) as usize] = sprite_normal[x as usize][y as usize];
                    }
                }
            }
        } else if !self.dead && self.enemy_type == 1 {
            for x in 0..7 {
                for y in 0..7 {
                    if x as f32 + self.x > -1.0 && x as f32 + self.x < 64.0 && y as f32 + self.y > -1.0 && y as f32 + self.y < 64.0 {
                        screen[(x as f32 + self.x) as usize][(y as f32 + self.y) as usize] = sprite_big[x as usize][y as usize];
                    }
                }
            }
        } else if !self.dead && self.enemy_type == 2 {
            for x in 0..5 {
                for y in 0..6 {
                    if x as f32 + self.x > -1.0 && x as f32 + self.x < 64.0 && y as f32 + self.y > -1.0 && y as f32 + self.y < 64.0 {
                        screen[(x as f32 + self.x) as usize][(y as f32 + self.y) as usize] = sprite_speed[x as usize][y as usize];
                    }
                }
            }
        }
    }

    pub fn update(&mut self, player: &mut Player, d: &RaylibDrawHandle) {
        if !self.dead {
            if player.x - 1.0 > self.x {
                self.x += self.speed * d.get_frame_time();
            } else if player.x + 1.0 < self.x {
                self.x -= self.speed * d.get_frame_time();
            }

            if player.y - 1.0 > self.y {
                self.y += self.speed * d.get_frame_time();
            } else if player.y + 1.0 < self.y {
                self.y -= self.speed * d.get_frame_time();
            }
        }

        if self.enemy_type == 0 {
            self.rectangle = Rectangle::new(self.x * 7.0 + 25.0, self.y * 7.0 + 25.0, 7.0 * 5.0, 7.0 * 5.0);
        } else if self.enemy_type == 1 {
            self.rectangle = Rectangle::new(self.x * 7.0 + 25.0, self.y * 7.0 + 25.0, 7.0 * 7.0, 7.0 * 7.0);
        } else if self.enemy_type == 2 {
            self.rectangle = Rectangle::new(self.x * 7.0 + 25.0, self.y * 7.0 + 25.0, 7.0 * 5.0, 7.0 * 6.0);
        }
    }
}
