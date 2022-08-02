/*
        T o d o  L i s t o

    C o d i n g
    TODO: TANK
    TODO: Enemies movement

    A r t
    TODO:
*/

mod player;
mod bullet;
mod enemy;
mod background;

use raylib::prelude::*;
use crate::bullet::Bullet;
use crate::enemy::Enemy;
use crate::player::Player;
use raylib::core::math::Rectangle;

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
}

fn main() {
    let (mut rl, thread) = init()
        .size(500, 500)
        .title("Lowrez Game Jam")
        .build();

    rl.set_target_fps(30);

    let mut screen = [[Color::new( 255, 255, 255, 255); 64] ;64];

    let mut player = Player::new();
    let mut bullets: Vec<Bullet> = vec![];
    let mut bullet_frame_buffer = 25;

    let mut test_enemy = Enemy::new(58.0, 58.0, 1.0);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        // DISPLAYOO
        player.display(&mut screen);
        test_enemy.display(&mut screen);

        test_enemy.update(&mut player);

        // displays bullets
        for mut i in &mut bullets {
            if i.x > -1.0 && i.x < 64.0 && i.y > -1.0 && i.y < 64.0 {
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
        for i in &bullets {
            if i.rectangle.check_collision_recs(&test_enemy.rectangle) {
                test_enemy.dead = true;
            }
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
    }
}
