use crate::chunk::{Chunk, ChunkMap};
use crate::components::{Player, Position};
use cgmath::MetricSpace;
use specs::{Join, ReadStorage, System, Write, WriteStorage};

pub struct RemoveChunks;
impl<'a> System<'a> for RemoveChunks {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'a, ChunkMap>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Chunk>,
    );

    fn run(&mut self, (mut chunk_map, player, positions, mut chunks): Self::SystemData) {
        let range = 8;
        let mut player_positions = Vec::new();

        for (_, position) in (&player, &positions).join() {
            let mut p = ChunkMap::f_coords_to_chunk_coords(position.0);
            p.y = 0;
            player_positions.push(p);
        }
        for (chunk_pos, chunk) in chunk_map.all() {
            let mut remove = true;
            for player_position in &player_positions {
                if chunk_pos.distance2(*player_position) < range * range {
                    remove = false;
                    break;
                }
            }
            if remove {
                println!("removing chunk {:?}", chunk_pos);
                chunks.remove(chunk);
                chunk_map.remove_chunk(chunk_pos);
            }
        }
    }
}
