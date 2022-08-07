use raylib::math::Rectangle;
use raylib::prelude::color::Color;

#[derive(Clone, Copy)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub facing: &'static str,
    pub speed: f32,
    pub lives: i8,
    pub rectangle: Rectangle,
}


impl Player {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            facing: "north",
            speed: 50.0,
            lives: 5,
            rectangle: Rectangle::new(0.0, 0.0, 0.0, 0.0),
        }
    }

    pub fn display(self, screen: &mut [[Color; 64]; 64]) {
        let north = [[Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255), Color::new(55, 148, 110, 255), Color::new(55, 148, 110, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(153, 229, 80, 255), Color::new(153, 229, 80, 255), Color::new(153, 229, 80, 255), Color::new(153, 229, 80, 255), Color::new(106, 190, 48, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255), Color::new(55, 148, 110, 255), Color::new(55, 148, 110, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)]];
        let south = [[Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255), Color::new(55, 148, 110, 255), Color::new(55, 148, 110, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(106, 190, 48, 255), Color::new(153, 229, 80, 255), Color::new(153, 229, 80, 255), Color::new(153, 229, 80, 255), Color::new(153, 229, 80, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255), Color::new(55, 148, 110, 255), Color::new(55, 148, 110, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)]];
        let west = [[Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(153, 229, 80, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(106, 190, 48, 255), Color::new(153, 229, 80, 255), Color::new(106, 190, 48, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255), Color::new(106, 190, 48, 255), Color::new(153, 229, 80, 255), Color::new(106, 190, 48, 255), Color::new(55, 148, 110, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255), Color::new(106, 190, 48, 255), Color::new(153, 229, 80, 255), Color::new(106, 190, 48, 255), Color::new(55, 148, 110, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(55, 148, 110, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)]];
        let east = [[Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(106, 190, 48, 255), Color::new(55, 148, 110, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255), Color::new(106, 190, 48, 255), Color::new(153, 229, 80, 255), Color::new(106, 190, 48, 255), Color::new(55, 148, 110, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(55, 148, 110, 255), Color::new(106, 190, 48, 255), Color::new(153, 229, 80, 255), Color::new(106, 190, 48, 255), Color::new(55, 148, 110, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(106, 190, 48, 255), Color::new(153, 229, 80, 255), Color::new(106, 190, 48, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)], [Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(153, 229, 80, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255), Color::new(110, 29, 29, 255)]];

        for x in 0..7 {
            for y in 0..7 {
                if x as f32 + self.x > -1.0 && x as f32 + self.x < 64.0 && y as f32 + self.y > -1.0 && y as f32 + self.y < 64.0 {
                    match self.facing {
                        "north" => screen[(x as f32 + self.x) as usize][(y as f32 + self.y) as usize] = north[x as usize][y as usize],
                        "south" => screen[(x as f32 + self.x) as usize][(y as f32 + self.y) as usize] = south[x as usize][y as usize],
                        "west" => screen[(x as f32 + self.x) as usize][(y as f32 + self.y) as usize] = west[x as usize][y as usize],
                        "east" => screen[(x as f32 + self.x) as usize][(y as f32 + self.y) as usize] = east[x as usize][y as usize],
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.rectangle = Rectangle::new(self.x * 7.0 + 25.0, self.y * 7.0 + 25.0, 8.0 * 7.0, 8.0 * 7.0);
    }
}
