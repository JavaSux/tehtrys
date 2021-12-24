#![allow(dead_code)]
#![feature(let_else, bool_to_option)]

use engine::{Engine, Matrix, Color};

mod engine;
mod interface;

fn main() {
    let mut matrix = Matrix::blank();
    matrix[(1, 1).into()] = Some(Color::Green);

    let engine = Engine::with_matrix(matrix);

    interface::run(engine);
}
