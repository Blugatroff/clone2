#![feature(stmt_expr_attributes)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]

mod blocks;
mod chunk;
mod chunk_map;
mod chunk_middle_ware;
mod components;
mod dir;
mod flat_middleware;
mod manager;
mod math_utils;
mod neighbours;
mod ray_intersection;
mod resources;
mod setup;
mod state;
mod systems;

use crate::state::State;

pub fn main() {
    let (window, event_loop) = simple_winit::create("clone_v2");

    simple_winit::start(State::new(&window), (window, event_loop));
}
