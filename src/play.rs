use crate::ai::eval;
use crate::game::{
    draw, gameover, hard_drop, hold, is_collision, landing, move_block, quit, rotate_left,
    rotate_right, Game, Position,
};
use getch_rs::{Getch, Key};
use std::{
    sync::{Arc, Mutex},
    thread, time,
};

fn sleep(milliseconds: u64) {
    thread::sleep(time::Duration::from_millis(milliseconds))
}

// 通常プレイ
pub fn normal() {
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
                // nミリ秒間スリープする
                let sleep_msec =
                    match 1000u64.saturating_sub((game.lock().unwrap().line as u64 / 10) * 100) {
                        0 => 100,
                        msec => msec,
                    };
                sleep(sleep_msec);

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
                break;
            }
            _ => (), // 何もしない
        }
    }

    quit();
}

// オートプレイ
pub fn auto() {
    // 自動化処理
    let _ = thread::spawn(|| loop {
        let mut game = Game::new();

        // 画面クリア
        println!("\x1b[2J\x1b[H\x1b[?25l");

        // フィールドを描画
        draw(&game);

        // 自然落下処理
        loop {
            // 100ミリ秒毎に自動で操作する
            sleep(100);

            let elite = eval(&game, &[100, 1, 10, 100]);
            game = elite;

            // エリート個体のブロックを落下
            if landing(&mut game).is_err() {
                // ブロックを生成できないならゲームオーバー
                gameover(&game);
                break;
            }
            draw(&game);
        }
    });

    // キー入力処理
    let g = Getch::new();
    loop {
        // キー入力待ち
        match g.getch() {
            Ok(Key::Char('q')) => {
                break;
            }
            _ => (), // 何もしない
        }
    }

    quit();
}
