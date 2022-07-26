use raylib::prelude::Color;

#[derive(Copy, Clone)]
pub struct Player {
    pub x: i32,
    pub y: i32
}

impl Player {
    pub fn get_sprite() -> [[Color; 4]; 4] {
        return [[Color::new(34, 32, 52, 255), Color::new(172, 50, 50, 255), Color::new(172, 50, 50, 255), Color::new(106, 190, 48, 255)], [Color::new(172, 50, 50, 255), Color::new(172, 50, 50, 255), Color::new(34, 32, 52, 255), Color::new(106, 190, 48, 255)], [Color::new(172, 50, 50, 255), Color::new(172, 50, 50, 255), Color::new(172, 50, 50, 255), Color::new(106, 190, 48, 255)], [Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255)]];
    }

    pub fn display_sprite(self, screen: &mut [[Color; 64]; 64]) {
        for x in 0..4 {
            for y in 0..4 {
                if self.x + x < 64 && self.y + y < 64 && self.x + x > -1 && self.y + y > -1 {
                    screen[(self.x + x) as usize][(self.y + y) as usize] = Player::get_sprite()[x as usize][y as usize];
                }
            }
        }
    }
}
