mod player;

use raylib::prelude::*;
use crate::player::Player;

fn main() {
    let (mut rl, thread) = init()
        .size(500, 500)
        .title("Lowrez Game Jam")
        .build();

    rl.set_target_fps(60);

    let mut screen = [[Color::new( 255, 255, 255, 255); 64] ;64];

    let mut player = Player{x: 0, y: 0};
    let mut half = false;

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);

        player.display_sprite(&mut screen);

        for x in 0..64 {
            for y in 0..64 {
                d.draw_rectangle(x as i32 * 7 + 25, y as i32 * 7 + 25, 7, 7, screen[x][y]);
            }
        }

        screen = [[Color::new(255, 255, 255, 255); 64]; 64];

        if half == false {
            if d.is_key_down(KeyboardKey::KEY_D) {
                player.x += 1;
            }

            if d.is_key_down(KeyboardKey::KEY_A) {
                player.x -= 1;
            }

            if d.is_key_down(KeyboardKey::KEY_W) {
                player.y -= 1;
            }

            if d.is_key_down(KeyboardKey::KEY_S) {
                player.y += 1;
            }
        }

        half = !half;
    }
}
