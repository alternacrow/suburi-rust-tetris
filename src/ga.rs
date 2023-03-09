use std::ops::Index;

use crate::ai::eval;
use crate::game::{landing, quit, Game};

// 遺伝子の種類
pub enum GenomeKind {
    Line,
    HeightMax,
    HeightDiff,
    DeadSpace,
}

// 遺伝子配列
pub type GenoSeq = [u8; 4];
impl Index<GenomeKind> for GenoSeq {
    type Output = u8;
    fn index(&self, kind: GenomeKind) -> &Self::Output {
        &self[kind as usize]
    }
}

// 遺伝子集合の個体数
const POPULATION: usize = 10;
// nライン消したら終了(1個体の実行終了条件)
const LINE_COUNT_MAX: usize = 256;

// 学習
pub fn learning() {
    let genos = rand::random::<[GenoSeq; POPULATION]>();
    for (i, geno) in genos.iter().enumerate() {
        let mut game = Game::new();
        // nライン消したら終了
        while game.line < LINE_COUNT_MAX {
            // 指定した遺伝子で評価後のエリート個体を取得
            let elite = eval(&game, geno);
            game = elite;
            // エリート個体のブロックを落下
            if landing(&mut game).is_err() {
                break;
            }
        }
        // 個体の最終スコアを表示
        println!("{i}: {:?} => {}", geno, game.score);
    }
    // 学習終了
    quit();
}
