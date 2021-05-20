use crate::chunk::{Chunk, ChunkMap};
use cgmath::Vector3;
use specs::{Entities, Join, Read, System, WriteStorage};

pub struct UpdateNeighbouringChunks;

impl<'a> System<'a> for UpdateNeighbouringChunks {
    type SystemData = (Read<'a, ChunkMap>, Entities<'a>, WriteStorage<'a, Chunk>);

    fn run(&mut self, (map, entities, mut chunks): Self::SystemData) {
        let mut to_update = vec![];
        for (_, chunk) in (&entities, &mut chunks).join() {
            for (dir, _) in chunk.update_neighbours.iter() {
                let p: Vector3<i32> = chunk.position + Vector3::from(dir);
                if let Some(chunk) = map.get_chunk(p) {
                    to_update.push(chunk);
                }
            }
            chunk.update_neighbours.clear();
        }
        for entity in to_update {
            if let Some(chunk) = chunks.get_mut(entity) {
                chunk.regenerate_mesh = true;
            }
        }
    }
}
