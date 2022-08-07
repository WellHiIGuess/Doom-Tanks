extern crate core;

mod player;
mod bullet;
mod enemy;
mod background;
mod enemy_spawner;
mod state;
mod start_screen;

use std::borrow::BorrowMut;
use rand::Rng;
use raylib::prelude::*;
use crate::bullet::Bullet;
use crate::enemy::Enemy;
use crate::enemy_spawner::EnemySpawner;
use crate::player::Player;
use crate::state::State;
use crate::start_screen::START_SCREEN;

fn main() {
    let (mut rl, thread) = init()
        .size(500, 500)
        .title("Lowrez Game Jam")
        .build();

    rl.set_target_fps(30);

    let mut a = RaylibAudio::init_audio_device();
    let kill_sound = Sound::load_sound("src/kill.wav");

    let mut screen = [[Color::new( 255, 255, 255, 255); 64] ;64];

    let mut phase = 1;
    let mut state = State::Playing;

    let mut player = Player::new();
    let mut damage_buffer = 50;
    let mut bullets: Vec<Bullet> = vec![];
    let mut bullet_frame_buffer = 25;

    let mut dead = false;

    // let mut test_enemy = Enemy::new(58.0, 58.0, 1.0);
    let mut enemy_spawner = EnemySpawner::new(50, 50, 3);

    while !rl.window_should_close() && !dead {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        let mut game = || {
            // DISPLAYOO
            player.display(&mut screen);

            let enemy_spawn_sprite = [[Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255), Color::new(110, 29, 29, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255)], [Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255)], [Color::new(110, 29, 29, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255), Color::new(110, 29, 29, 255)], [Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255)], [Color::new(215, 123, 186, 255), Color::new(118, 66, 138, 255), Color::new(110, 29, 29, 255), Color::new(118, 66, 138, 255), Color::new(215, 123, 186, 255)]];

            let mut temp_spawner = &mut enemy_spawner;

            for x in 0..5 {
                for y in 0..5 {
                    if x + temp_spawner.x - 2 > -1 && x + temp_spawner.x - 2 < 64 && y + temp_spawner.y - 2 > -1 && y + temp_spawner.y - 2 < 64 {
                        screen[(x + temp_spawner.x) as usize - 2][(y + temp_spawner.y) as usize - 2] = enemy_spawn_sprite[x as usize][y as usize];
                    }
                }
            }

            // displays and updates enemies
            for i in &mut enemy_spawner.enemies {
                i.display(&mut screen);
                i.update(&mut player, &d);
            }

            // displays health
            for i in 0..player.lives {
                display_heart(i as i32 * 3, &mut screen);
            }

            enemy_spawner.update();
            player.update();

            // displays bullets
            for i in &mut bullets {
                if i.x > -1.0 && i.x < 64.0 && i.y > -1.0 && i.y < 64.0 && !i.dead {
                    screen[i.x as usize][i.y as usize] = Color::new(255, 255, 0, 255);
                    unsafe { i.update(d.get_frame_time()); }
                }
            }

            // optimizes bullets
            if bullets.len() > 10 {
                bullets.remove(0);
            }

            // drawing screen
            for x in 0..64 {
                for y in 0..64 {
                    d.draw_rectangle(x as i32 * 7 + 25, y as i32 * 7 + 25, 7, 7, screen[x][y]);
                }
            }

            // bullet collisions
            let mut dead_enemies = 0;

            for i in &mut bullets {
                for j in &mut enemy_spawner.enemies {
                    if i.rectangle.check_collision_recs(&j.rectangle) && !i.dead && !j.dead {
                        j.health -= 1;
                        i.dead = true;

                        if j.health == 0 {
                            j.dead = true;

                            a.play_sound(kill_sound.as_ref().unwrap());
                        }
                    }
                }
            }

            // player collisions
            for i in &mut enemy_spawner.enemies {
                if player.rectangle.check_collision_recs(&i.rectangle) && !i.dead && damage_buffer == 50 {
                    player.lives -= 1;
                    damage_buffer -= 1;
                }
            }

            if damage_buffer < 50 {
                damage_buffer -= 1;
            }

            if damage_buffer == 0 {
                damage_buffer = 50;
            }

            // makes sure enemies keep spawning once you kill all of a spawners enemies
            for j in &mut enemy_spawner.enemies {
                if j.dead {
                    dead_enemies += 1;
                }
            }

            if dead_enemies == enemy_spawner.max_enemies {
                enemy_spawner.enemies = vec![];
                phase += 1;
                enemy_spawner.max_enemies = 3 * phase;
            }

            player_controls(&mut player, &mut d);

            if d.is_key_down(KeyboardKey::KEY_L) && bullet_frame_buffer == 25 {
                bullets.push(Bullet::new(player.x + 3.0, player.y + 3.0, player.facing));
                bullet_frame_buffer -= 1;
            }

            if bullet_frame_buffer != 25 {
                bullet_frame_buffer -= 1;
            }

            if bullet_frame_buffer == 0 {
                bullet_frame_buffer = 25;
            }

            unsafe { screen = background::BACKGROUND; }

            if player.lives == 0 {
                dead = true;
            }
        };

        if state == State::Playing {
            game();
        } else if state == State::Start {
            start(&mut screen, &mut d);
        }
    }
}


fn start(screen: &mut [[Color; 64]; 64], d: &mut RaylibDrawHandle) {
    *screen = START_SCREEN;

    // drawing screen
    for x in 0..64 {
        for y in 0..64 {
            d.draw_rectangle(x as i32 * 7 + 25, y as i32 * 7 + 25, 7, 7, screen[x][y]);
        }
    }
}

fn player_controls(player: &mut Player, d: &mut RaylibDrawHandle) {
    if d.is_key_down(KeyboardKey::KEY_S) {
        if player.facing == "north" {
            player.y += player.speed * d.get_frame_time();
        } else if player.facing == "west" {
            player.x += player.speed * d.get_frame_time();
        } else if player.facing == "east" {
            player.x -= player.speed * d.get_frame_time();
        } else if player.facing == "south" {
            player.y -= player.speed * d.get_frame_time();
        }
    }

    if d.is_key_down(KeyboardKey::KEY_W) {
        if player.facing == "north" {
            player.y -= player.speed * d.get_frame_time();
        } else if player.facing == "west" {
            player.x -= player.speed * d.get_frame_time();
        } else if player.facing == "east" {
            player.x += player.speed * d.get_frame_time();
        } else if player.facing == "south" {
            player.y += player.speed * d.get_frame_time();
        }
    }

    if d.is_key_pressed(KeyboardKey::KEY_D) {
        if player.facing == "north" {
            player.facing = "east";
        } else if player.facing == "east" {
            player.facing = "south";
        } else if player.facing == "south" {
            player.facing = "west";
        } else if player.facing == "west" {
            player.facing = "north";
        }
    }

    if d.is_key_pressed(KeyboardKey::KEY_A) {
        if player.facing == "north" {
            player.facing = "west";
        } else if player.facing == "east" {
            player.facing = "north";
        } else if player.facing == "west" {
            player.facing = "south";
        } else if player.facing == "south" {
            player.facing = "east";
        }
    }

    // makes player go to other side of screen
    if player.x >= 57.0 {
        player.x = 57.0;
    } else if player.x <= 0.0 {
        player.x = 0.0;
    }

    if player.y >= 57.0 {
        player.y = 57.0;
    } else if player.y <= 0.0 {
        player.y = 0.0;
    }
}

fn display_heart(x_pos: i32, screen: &mut [[Color; 64]; 64]) {
    let heart_sprite = [[Color::PINK, Color::PINK], [Color::PINK, Color::PINK]];

    for x in 0..2 {
        for y in 0..2 {
            screen[(x + x_pos) as usize][(y + 59) as usize] = heart_sprite[x as usize][y as usize];
        }
    }
}
