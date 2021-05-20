use crate::chunk::{Chunk, ChunkMap};
use crate::components::Player;
use crate::{blocks::Block, components::LookedAt};
use simple_winit::input::Input;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct BreakBlocks<'a>(pub &'a mut Input);

impl<'a> System<'a> for BreakBlocks<'_> {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, ChunkMap>,
        WriteStorage<'a, Chunk>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, LookedAt>,
    );

    fn run(&mut self, (chunk_map, mut chunks, players, looked_at): Self::SystemData) {
        for (_, looked_at) in (&players, &looked_at).join() {
            if self
                .0
                .button_pressed(simple_winit::input::MouseButton::Left)
            {
                if let Some(chunk) = chunk_map.get_chunk_from_coords(looked_at.coords) {
                    let block_coord =
                        ChunkMap::coords_to_chunk_coords_and_block(looked_at.coords).1;
                    let chunk = chunks.get_mut(chunk).unwrap();
                    dbg!(block_coord);
                    dbg!(chunk.get_block(block_coord));
                    chunk.set_block(block_coord, Block::Empty);
                }
            }
        }
    }
}
