#![allow(dead_code)]
#![feature(let_else, bool_to_option)]

mod engine;
mod interface;

use engine::{Engine, Matrix, Color, piece::Kind as PieceKind};

fn main() {
    let mut matrix = Matrix::blank();
    matrix[(1, 1).into()] = Some(Color::Green);

    let mut engine = Engine::with_matrix(matrix);
    engine.DEBUG_test_cursor(PieceKind::T, (5, 5).into());

    interface::run(engine);
}
