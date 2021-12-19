#![allow(dead_code)]
#![feature(let_else)]

use engine::Engine;
use interface::Interface;


mod engine;
mod interface;

fn main() {
    let engine = Engine::new();

    Interface::run(engine)
}
