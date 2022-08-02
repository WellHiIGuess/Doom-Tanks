use raylib::color::Color;
use raylib::prelude::Rectangle;
use crate::Player;

#[derive(Copy, Clone)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub rectangle: Rectangle,
    pub dead: bool,
}

impl Enemy {
    pub fn new(x: f32, y: f32, speed: f32) -> Self {
        Self {
            x,
            y,
            speed,
            rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            dead: false,
        }
    }

    pub fn display(&mut self, screen: &mut [[Color; 64]; 64]) {
        let sprite = [[Color::new(110, 29, 29, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(110, 29, 29, 255)], [Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(229, 204, 64, 255), Color::new(229, 204, 64, 255), Color::new(118, 66, 138, 255)], [Color::new(215, 123, 186, 255), Color::new(251, 242, 54, 255), Color::new(166, 48, 30, 255), Color::new(166, 48, 30, 255), Color::new(251, 242, 54, 255)], [Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(229, 204, 64, 255), Color::new(229, 204, 64, 255), Color::new(118, 66, 138, 255)], [Color::new(110, 29, 29, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(118, 66, 138, 255), Color::new(110, 29, 29, 255)]];

        if !self.dead {
            for x in 0..5 {
                for y in 0..5 {
                    screen[(x as f32 + self.x) as usize][(y as f32 + self.y) as usize] = sprite[x as usize][y as usize];
                }
            }
        }
    }

    pub fn update(&mut self, player: &mut Player) {
        if !self.dead {
            self.x += (player.x - self.x) / 100.0;
            self.y += (player.y - self.y) / 100.0;
        }

        self.rectangle = Rectangle::new(self.x * 7.0 + 25.0, self.y * 7.0 + 25.0, 7.0 * 5.0, 7.0 * 5.0);
    }
}
