use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::intcode::vm::IntCodeMachine;

mod renderer;
mod sim;
mod state;

fn count_blocks_and_measure_board(tape: &[i64]) -> (usize, usize, usize) {
    let map_data = IntCodeMachine::run_all(tape, &[]);
    let mut game = state::Game::new();
    game.load_state_stream(&map_data);
    (
        game.count_tiles(state::Tile::Block),
        game.width(),
        game.height(),
    )
}

pub fn main() {
    let tape: Vec<i64> = std::fs::read_to_string("data/day13.txt")
        .unwrap()
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();

    let (result0, width, height) = count_blocks_and_measure_board(&tape);

    let (tick_tx, tick_rx) = mpsc::channel::<()>();

    let shared_state = Arc::new(Mutex::new(state::Game::new()));

    let renderer_state = shared_state.clone();
    let sim_state = shared_state.clone();

    let render_thread =
        thread::spawn(move || renderer::run(width as u32, height as u32, tick_rx, renderer_state));
    let sim_thread = thread::spawn(move || sim::run(&tape, tick_tx, sim_state));

    render_thread.join().unwrap();
    sim_thread.join().unwrap();

    let result1 = shared_state.lock().unwrap().score();

    println!("{} {}", result0, result1);
}
