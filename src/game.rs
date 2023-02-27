use crate::block::{BlockKind, BlockShape, BLOCKS, BLOCK_SIZE};

pub const FIELD_WIDTH: usize = 11 + 2 + 2; // フィールド+壁+番兵
pub const FIELD_HEIGHT: usize = 20 + 1 + 1; // フィールド+壁+番兵
pub type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn init() -> Position {
        Position { x: 5, y: 0 }
    }
}

pub struct Game {
    pub field: Field,
    pub pos: Position,
    pub block: BlockShape,
}

const DEFAULT_FIELD: Field = [
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];

impl Game {
    pub fn new() -> Game {
        Game {
            field: DEFAULT_FIELD,
            pos: Position::init(),
            block: BLOCKS[rand::random::<BlockKind>() as usize],
        }
    }
}

// フィールドを描画する
#[allow(clippy::needless_range_loop)]
pub fn draw(Game { field, pos, block }: &Game) {
    // 描画用フィールドの生成
    let mut field_buf: Field = *field;

    // 描画用フィールドにブロックの情報を書き込む
    for y in 0..BLOCK_SIZE {
        for x in 0..BLOCK_SIZE {
            if block[y][x] == 1 {
                field_buf[y + pos.y][x + pos.x] = 1;
            }

            //
        }
    }

    // フィールドを描画
    println!("\x1b[H"); // カーソルを先頭に移動
    for y in 0..FIELD_HEIGHT - 1 {
        for x in 1..FIELD_WIDTH - 1 {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

// ブロックがフィールドに衝突する場合は`true`を返す
pub fn is_collision(field: &Field, pos: &Position, block: &BlockShape) -> bool {
    for y in 0..BLOCK_SIZE {
        for x in 0..BLOCK_SIZE {
            if y + pos.y >= FIELD_HEIGHT || x + pos.x >= FIELD_WIDTH {
                continue;
            }

            if field[y + pos.y][x + pos.x] & block[y][x] == 1 {
                return true;
            }
        }
    }

    false
}

// ブロックを指定座標へ移動できるなら移動させる
pub fn move_block(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, &game.block) {
        game.pos = new_pos;
    }
}

// ブロックをフィールドに固定する
pub fn fix_block(Game { field, pos, block }: &mut Game) {
    for y in 0..BLOCK_SIZE {
        for x in 0..BLOCK_SIZE {
            if block[y][x] == 1 {
                field[y + pos.y][x + pos.x] = 1;
            }
        }
    }
}

// 消せるラインがあれば削除し、段を下げる
pub fn erase_line(field: &mut Field) {
    // ラインの削除処理
    for y in 1..FIELD_HEIGHT - 2 {
        let mut can_erase = true;
        for x in 2..FIELD_WIDTH - 2 {
            if field[y][x] == 0 {
                can_erase = false;
                break;
            }
        }

        if can_erase {
            for y2 in (2..=y).rev() {
                field[y2] = field[y2 - 1];
            }
        }
    }
}

pub fn spwan_block(game: &mut Game) -> Result<(), ()> {
    // posの座標を初期位置へ
    game.pos = Position::init();

    // ブロックをランダム生成
    game.block = BLOCKS[rand::random::<BlockKind>() as usize];

    // 衝突チェック
    if is_collision(&game.field, &game.pos, &game.block) {
        Err(())
    } else {
        Ok(())
    }
}

// 左に90度回転する
#[allow(clippy::needless_range_loop)]
pub fn rotate_left(game: &mut Game) {
    //
    let mut new_shape: BlockShape = Default::default();
    for y in 0..BLOCK_SIZE {
        for x in 0..BLOCK_SIZE {
            new_shape[4 - 1 - x][y] = game.block[y][x];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.block = new_shape;
    }
}

// 右に90度回転する
#[allow(clippy::needless_range_loop)]
pub fn rotate_right(game: &mut Game) {
    //
    let mut new_shape: BlockShape = Default::default();
    for y in 0..BLOCK_SIZE {
        for x in 0..BLOCK_SIZE {
            new_shape[y][x] = game.block[4 - 1 - x][y];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.block = new_shape;
    }
}

// ゲームオーバー処理
pub fn gameover(game: &Game) {
    draw(game);
    print!("GAMEOVER");
    quit();
}

// 終了処理
pub fn quit() -> ! {
    // カーソルを再表示
    println!("\x1b[?25h");
    std::process::exit(0);
}
