mod block;
mod game;

use game::{draw, is_collision, Game, Position};
use getch_rs::{Getch, Key};
use std::{
    sync::{Arc, Mutex},
    thread, time,
};

use crate::game::{
    gameover, hard_drop, hold, landing, move_block, quit, rotate_left, rotate_right,
};

fn sleep(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds))
}

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");
    // フィールドを描画
    draw(&game.lock().unwrap());

    // 自然落下処理
    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || {
            loop {
                // 1秒スリープする
                sleep(1000);

                // 自然落下
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };

                if !is_collision(&game.field, &new_pos, &game.block) {
                    // posの座標を更新
                    game.pos = new_pos;
                } else {
                    if landing(&mut game).is_err() {
                        // ブロックを生成できないならゲームオーバー
                        gameover(&game);
                    }
                }

                // フィールドを描画
                draw(&game);
            }
        });
    }

    let g = Getch::new();
    loop {
        // キー入力待ち
        match g.getch() {
            Ok(Key::Char(' ')) => {
                // ホールド
                let mut game = game.lock().unwrap();
                hold(&mut game);
                draw(&game);
            }
            Ok(Key::Up) => {
                // ハードドロップ
                let mut game = game.lock().unwrap();
                hard_drop(&mut game);
                if landing(&mut game).is_err() {
                    // ブロックを生成できないならゲームオーバー
                    gameover(&game);
                }
                draw(&game);
            }
            Ok(Key::Left) => {
                // 左移動
                let mut game = game.lock().unwrap();

                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                // 右移動
                let mut game = game.lock().unwrap();

                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                // 下移動
                let mut game = game.lock().unwrap();

                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Char('z')) => {
                // 左回転
                let mut game = game.lock().unwrap();
                rotate_left(&mut game);
                draw(&game);
            }
            Ok(Key::Char('x')) => {
                // 右回転
                let mut game = game.lock().unwrap();
                rotate_right(&mut game);
                draw(&game);
            }
            Ok(Key::Char('q')) => {
                quit();
            }
            _ => (), // 何もしない
        }
    }
}
