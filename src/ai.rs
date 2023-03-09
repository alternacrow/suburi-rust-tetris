use crate::block::block_kind;
use crate::game::{
    fix_block, hard_drop, move_block, rotate_right, Field, Game, Position, FIELD_HEIGHT,
    FIELD_WIDTH,
};

// 評価して、一番優秀な個体を返す
pub fn eval(game: &Game) -> Game {
    // エリートブロック (Game, line, height_max)
    let mut elite = (game.clone(), 0, FIELD_HEIGHT);

    // 全回転
    for rotate_count in 0..=3 {
        let mut game = game.clone();
        for _ in 0..=rotate_count {
            // 回転処理
            rotate_right(&mut game);
        }

        // 全横移動
        for dx in -4..=5 {
            let mut game = game.clone();
            // 移動処理
            let new_pos = Position {
                x: match game.pos.x as isize + dx {
                    (..=0) => 0,
                    x => x as usize,
                },
                y: game.pos.y,
            };
            move_block(&mut game, new_pos);
            hard_drop(&mut game);
            fix_block(&mut game);

            // インプット情報の取得
            let line = erase_line_count(&game.field); // 消せるライン数
            let height_max = field_height_max(&game.field); // フィールドの高さ

            // インプット情報を評価
            if line >= elite.1 && height_max <= elite.2 {
                // 一番良い個体を記録
                elite.0 = game;
                elite.1 = line;
                elite.2 = height_max;
            }
        }
    }
    elite.0
}

// 消せるライン数を返す
fn erase_line_count(field: &Field) -> usize {
    let mut count = 0;
    for y in 1..FIELD_HEIGHT - 2 {
        let mut can_erase = true;
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] == block_kind::NONE {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            count += 1;
        }
    }
    count
}

/**
 * フィールドの一番高いブロックの高さを返す
 * ブロックが何もないときは0を返す
 * ブロックが積みあがっていくにつれ、数値は増える
 */
fn field_height_max(field: &Field) -> usize {
    for y in 1..FIELD_HEIGHT - 2 {
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] != block_kind::NONE {
                return FIELD_HEIGHT - y - 1;
            }
        }
    }
    unreachable!()
}
