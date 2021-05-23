use crate::chunk::Chunk;
use crate::{
    chunk_map::ChunkMap,
    components::{LookedAt, Player, Position, Rotation},
};
use cgmath::Vector3;
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

pub struct LookingAtSystem;
impl<'a> System<'a> for LookingAtSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, ChunkMap>,
        ReadStorage<'a, Chunk>,
        ReadStorage<'a, Player>,
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rotation>,
        WriteStorage<'a, LookedAt>,
    );

    fn run(
        &mut self,
        (chunk_map, chunks, players, entities, positions, rotations, mut looked_at): Self::SystemData,
    ) {
        for (_, entity, pos, rot) in (&players, &entities, &positions, &rotations).join() {
            if let Some(target) =
                chunk_map.ray_intersection(&chunks, pos.0, (rot.0 * Vector3::unit_x()) * 5.0)
            {
                if let Some(looked_at_block) = looked_at.get_mut(entity) {
                    *looked_at_block = target;
                } else {
                    looked_at.insert(entity, target).unwrap();
                }
            } else {
                looked_at.remove(entity);
            }
        }
    }
}
