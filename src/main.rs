#![allow(dead_code)]
#![feature(let_else, bool_to_option, is_sorted, array_chunks)]

mod engine;
mod interface;

use engine::{Engine, Matrix, Color, piece::Kind as PieceKind};

fn main() {
    let mut matrix = Matrix::blank();
    for col in 0..=6 {
        matrix[(col, 0).into()] = Some(Color::Green);
    }

    let mut engine = Engine::with_matrix(matrix);
    engine.DEBUG_test_cursor(PieceKind::T, (5, 19).into());

    interface::run(engine);
}
