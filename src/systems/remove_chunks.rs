use crate::components::{Player, Position};
use crate::{chunk::Chunk, chunk_map::ChunkMap};
use cgmath::MetricSpace;
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

pub struct RemoveChunks;
impl<'a> System<'a> for RemoveChunks {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Chunk>,
        Entities<'a>,
        Write<'a, ChunkMap>,
    );

    fn run(&mut self, (player, positions, chunks, entities, mut chunk_map): Self::SystemData) {
        let range = 8;
        let mut player_positions = Vec::new();

        for (_, position) in (&player, &positions).join() {
            let mut p = ChunkMap::f_coords_to_chunk_coords(position.0);
            p.y = 0;
            player_positions.push(p);
        }
        let mut chunks_to_remove = Vec::new();
        for (entity, chunk) in (&entities, &chunks).join() {
            let mut remove = true;
            let position = chunk.position;
            for player_position in &player_positions {
                if position.distance2(*player_position) < range * range {
                    remove = false;
                }
            }
            if remove {
                chunks_to_remove.push(entity);
            }
        }
        for chunk in chunks_to_remove {
            println!("deleting chunk");
            chunk_map.remove_chunk(chunks.get(chunk).unwrap().position);
            entities.delete(chunk).unwrap();
        }
    }
}
