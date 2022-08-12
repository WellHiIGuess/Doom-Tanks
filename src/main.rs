#![windows_subsystem = "windows"]

extern crate core;

mod player;
mod bullet;
mod enemy;
mod background;
mod enemy_spawner;
mod state;
mod start_screen;
mod death_screen;

use std::borrow::BorrowMut;
use std::ops::{Deref, Index};
use std::path::PathBuf;
use rand::Rng;
use raylib::prelude::*;
use crate::background::BACKGROUND;
use crate::bullet::Bullet;
use crate::death_screen::DEATH_SCREEN;
use crate::enemy::Enemy;
use crate::enemy_spawner::EnemySpawner;
use crate::player::Player;
use crate::state::State;
use crate::start_screen::START_SCREEN;
use crate::State::*;

fn main() {
    let (mut rl, thread) = init()
        .size(500, 500)
        .title("Lowrez Game Jam")
        .build();

    rl.set_target_fps(30);

    let length = std::env::current_exe().unwrap().to_str().unwrap().len();

    let mut a = RaylibAudio::init_audio_device();
    // let kill_sound = Sound::load_sound("src/resources/kill.wav");
    let kill_sound = Sound::load_sound(&*(std::env::current_exe().unwrap().to_str().unwrap()[..length - 8].to_owned() + "src/resources/kill.wav"));
    // let player_death = Sound::load_sound("src/resources/player_death.wav");
    let player_death = Sound::load_sound(&*(std::env::current_exe().unwrap().to_str().unwrap()[..length - 8].to_owned() + "src/resources/player_death.wav"));
    // let laser_sound = Sound::load_sound("src/resources/laserShoot.wav");
    let laser_sound = Sound::load_sound(&*(std::env::current_exe().unwrap().to_str().unwrap()[..length - 8].to_owned() + "src/resources/laserShoot.wav"));
    // let rocket = Sound::load_sound("src/resources/rocket.wav");
    let rocket = Sound::load_sound(&*(std::env::current_exe().unwrap().to_str().unwrap()[..length - 8].to_owned() + "src/resources/rocket.wav"));
    let hurt = Sound::load_sound(&*(std::env::current_exe().unwrap().to_str().unwrap()[..length - 8].to_owned() + "src/resources/hurt.wav"));
    // let hurt = Sound::load_sound("src/resources/hurt.wav");
    // let phase_10 = Sound::load_sound("src/resources/phase_10.wav");
    let phase_10= Sound::load_sound(&*(std::env::current_exe().unwrap().to_str().unwrap()[..length - 8].to_owned() + "src/resources/phase_10.wav"));
    let mut notified = false;

    let font = rl.load_font(&thread, &*(std::env::current_exe().unwrap().to_str().unwrap()[..length - 8].to_owned() + "src/resources/font.ttf"));

    let mut screen = [[Color::new( 255, 255, 255, 255); 64] ;64];

    let mut phase = 0;
    let mut state = Start;

    let mut player = Player::new();
    let mut damage_buffer = 50;
    let mut bullets: Vec<Bullet> = vec![];
    let mut bullet_frame_buffer = 25;
    let mut const_bullet_frame_buffer = 25;
    let mut points = 0;

    let mut point_buffer = 30;

    let mut dead = false;

    // let mut test_enemy = Enemy::new(58.0, 58.0, 1.0);
    let mut enemy_spawner = EnemySpawner::new(50, 50, 3);

    while !rl.window_should_close() {
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
                    if phase >= 10 {
                        screen[i.x as usize][i.y as usize] = Color::new(255, 155, 0, 255);
                        if i.x + 1.0 > -1.0 && i.x + 1.0 < 64.0 && i.y + 1.0 > -1.0 && i.y + 1.0 < 64.0 && !i.dead { screen[(i.x + 1.0) as usize][(i.y + 1.0) as usize] = Color::new(255, 155, 0, 255); }
                        if i.x - 1.0 > -1.0 && i.x - 1.0 < 64.0 && i.y - 1.0 > -1.0 && i.y - 1.0 < 64.0 && !i.dead { screen[(i.x - 1.0) as usize][(i.y - 1.0) as usize] = Color::new(255, 155, 0, 255); }
                    } else {
                        screen[i.x as usize][i.y as usize] = Color::new(255, 255, 0, 255);
                    }

                    unsafe { i.update(d.get_frame_time(), phase); }
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

            d.draw_text_ex(font.as_ref().unwrap(), points.to_string().as_str(), Vector2::new(25.0, 25.0), 27 as f32, 0.0, Color::WHITE);

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
                            points += 10;
                        }
                    }
                }
            }

            // player collisions
            for i in &mut enemy_spawner.enemies {
                if player.rectangle.check_collision_recs(&i.rectangle) && !i.dead && damage_buffer == 50 {
                    player.lives -= 1;
                    damage_buffer -= 1;
                    a.play_sound(hurt.as_ref().unwrap());
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

            if phase >= 10 {
                const_bullet_frame_buffer = 15;
            }

            if phase == 10 && !notified {
                a.play_sound(phase_10.as_ref().unwrap());
                notified = true;
            }

            if d.is_key_down(KeyboardKey::KEY_L) && bullet_frame_buffer == const_bullet_frame_buffer {
                if phase >= 10 {
                    bullets.push(Bullet::new(player.x + 3.0, player.y + 3.0, player.facing, 1));
                } else {
                    bullets.push(Bullet::new(player.x + 3.0, player.y + 3.0, player.facing, 0));
                }

                a.play_sound(laser_sound.as_ref().unwrap());
                a.play_sound(rocket.as_ref().unwrap());
                bullet_frame_buffer -= 1;
            }

            if bullet_frame_buffer != const_bullet_frame_buffer {
                bullet_frame_buffer -= 1;
            }

            if bullet_frame_buffer == 0 {
                bullet_frame_buffer = const_bullet_frame_buffer;
            }

            unsafe { screen = background::BACKGROUND; }

            if player.lives == 0 {
                a.play_sound(player_death.as_ref().unwrap());
                dead = true;
            }

            if point_buffer > 0 {
                point_buffer -= 1;
            } else {
                points += 1;
                point_buffer = 30;
            }
        };


        match state {
            Playing => game(),
            Start => start(&mut screen, &mut d, &mut state),
            Started => started(&mut screen, &mut d, font.as_ref().unwrap(), &mut state),
            Tutorial => tutorial(&mut screen, &mut d, font.as_ref().unwrap(), &mut state),
            Dead => death_screen(&mut screen, &mut d, &mut state, &mut dead, &mut player, &mut enemy_spawner),
            _ => {}
        }

        if *&dead {
            state = Dead;
            phase = 0;
            points = 0;
        }
    }
}

fn start(screen: &mut [[Color; 64]; 64], d: &mut RaylibDrawHandle, state: &mut State) {
    *screen = START_SCREEN;

    // drawing screen
    for x in 0..64 {
        for y in 0..64 {
            d.draw_rectangle(x as i32 * 7 + 25, y as i32 * 7 + 25, 7, 7, screen[x][y]);
        }
    }

    if d.is_key_pressed(KeyboardKey::KEY_Q) {
        *state = Started;
    }
}

fn started(screen: &mut [[Color; 64]; 64], d: &mut RaylibDrawHandle, font: &Font, state: &mut State) {
    *screen = BACKGROUND;

    // drawing screen
    for x in 0..64 {
        for y in 0..64 {
            d.draw_rectangle(x as i32 * 7 + 25, y as i32 * 7 + 25, 7, 7, screen[x][y]);
        }
    }

    d.draw_text_ex(font, "PLAY", Vector2::new(95.0, 95.0), 27.0, 0.0, Color::WHITE);
    d.draw_text_ex(font, "TUTORIAL", Vector2::new(95.0, 165.0), 27.0, 0.0, Color::WHITE);

    if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        if d.get_mouse_y() > 94 && d.get_mouse_y() < 128 {
            *state = Playing;
        } else if d.get_mouse_y() > 164 && d.get_mouse_y() < 192 {
            *state = Tutorial;
        }
    }
}

fn tutorial(screen: &mut [[Color; 64]; 64], d: &mut RaylibDrawHandle, font: &Font, state: &mut State) {
    *screen = BACKGROUND;

    // drawing screen
    for x in 0..64 {
        for y in 0..64 {
            d.draw_rectangle(x as i32 * 7 + 25, y as i32 * 7 + 25, 7, 7, screen[x][y]);
        }
    }

    d.draw_text_ex(font, "PLAY", Vector2::new(95.0, 95.0), 27.0, 0.0, Color::WHITE);
    d.draw_text_ex(font, "WS - Forward \nand back\n\nAD - Turn\n\nL - SHOOT!", Vector2::new(95.0, 165.0), 27.0, 0.0, Color::WHITE);

    if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
        if d.get_mouse_y() > 94 && d.get_mouse_y() < 128 {
            *state = Playing;
        }
    }
}

fn death_screen(screen: &mut [[Color; 64]; 64], d: &mut RaylibDrawHandle, state: &mut State, dead: &mut bool, player: &mut Player, enemy_spawner: &mut EnemySpawner) {
    *screen = DEATH_SCREEN;

    // drawing screen
    for x in 0..64 {
        for y in 0..64 {
            d.draw_rectangle(x as i32 * 7 + 25, y as i32 * 7 + 25, 7, 7, screen[x][y]);
        }
    }

    if d.is_key_pressed(KeyboardKey::KEY_Q) {
        *dead = false;
        *state = Playing;
        player.x = 0.0; player.y = 0.0;
        player.lives = 5;
        player.facing = "north";
        enemy_spawner.enemies = vec![];
        enemy_spawner.x = 50;
        enemy_spawner.y = 50;
        enemy_spawner.max_enemies = 3;
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
