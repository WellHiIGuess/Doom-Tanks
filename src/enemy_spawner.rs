use rand::Rng;
use raylib::color::Color;
use crate::Enemy;

const BUFFER: usize = 50;
// const BUFFER: usize = 1;

pub struct EnemySpawner {
    pub x: i32,
    pub y: i32,
    pub max_enemies: usize,
    pub enemies: Vec<Enemy>,
    pub spawn_buffer: usize,
}

impl EnemySpawner {
    pub fn new(x: i32, y: i32, max_enemies: usize) -> Self {
        Self {
            x,
            y,
            max_enemies,
            enemies: vec![],
            spawn_buffer: BUFFER,
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();

        if self.spawn_buffer == BUFFER && self.enemies.len() < self.max_enemies {
            self.enemies.push(Enemy::new(self.x as f32, self.y as f32));
            self.spawn_buffer -= 1;
            self.x = rng.gen_range(0..63);
            self.y = rng.gen_range(0..63);
        } else {
            self.spawn_buffer -= 1;
        }

        if self.spawn_buffer == 0 {
            self.spawn_buffer = BUFFER;
        }
    }
}
