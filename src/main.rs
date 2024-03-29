mod block;
mod game;

use game::*;
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    // field を描画
    draw(&game.lock().unwrap());

    // 自然落下
    {
        let game = Arc::clone(&game);

        let _ = thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(1000));
            let mut game = game.lock().unwrap();

            let new_pos = Position {
                x: game.pos.x,
                y: game.pos.y + 1,
            };
            if !is_collision(&game.field, &new_pos, game.block) {
                game.pos = new_pos;
            } else {
                fix_block(&mut game);
                erase_line(&mut game.field);
                game.pos = Position::init();
                game.block = rand::random();
            }
            draw(&game);
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();

                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or_else(|| game.pos.x),
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();

                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Char('q')) => {
                println!("\x1b[?25h");
                return;
            }
            _ => (),
        }
    }
}
