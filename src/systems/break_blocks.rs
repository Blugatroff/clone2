use std::sync::{Arc, Mutex};

use crate::chunk::{Chunk, ChunkMap};
use crate::components::Player;
use crate::{blocks::Block, components::LookedAt};
use simple_winit::input::Input;
use specs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};

pub struct BreakBlocks;

impl<'a> System<'a> for BreakBlocks {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, ChunkMap>,
        WriteStorage<'a, Chunk>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, LookedAt>,
        ReadExpect<'a, Arc<Mutex<Input>>>,
        ReadExpect<'a, ton::Player>
    );

    fn run(&mut self, (chunk_map, mut chunks, players, looked_at, input, player): Self::SystemData) {
        let input = input.lock().unwrap();
        for (_, looked_at) in (&players, &looked_at).join() {
            if input.button_pressed(simple_winit::input::MouseButton::Left) {
                if let Some(chunk) = chunk_map.get_chunk_from_coords(looked_at.coords) {
                    let block_coord =
                        ChunkMap::coords_to_chunk_coords_and_block(looked_at.coords).1;
                    let chunk = chunks.get_mut(chunk).unwrap();

                    match chunk.get_block(block_coord) {
                        Block::Empty => {}
                        Block::Dirt => {
                            let sand = ton::PlayableSfxr::load_from_json(&std::fs::read_to_string("./dirt.json").unwrap()).unwrap();
                            player.play(sand).unwrap().detach();
                        }
                        Block::Stone => {}
                        Block::Sand => {
                            let sand = ton::PlayableSfxr::load_from_json(&std::fs::read_to_string("./sand.json").unwrap()).unwrap();
                            player.play(sand).unwrap().detach();
                        }
                        Block::Water => {}
                        Block::Grass => {
                            let sand = ton::PlayableSfxr::load_from_json(&std::fs::read_to_string("./dirt.json").unwrap()).unwrap();
                            player.play(sand).unwrap().detach();
                        }
                    }
                    chunk.set_block(block_coord, Block::Empty);
                }
            }
        }
    }
}
