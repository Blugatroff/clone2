use crate::components::{LookedAt, Player, Position, Rotation};
use crate::{
    chunk::{Chunk, ChunkMap},
    components::LookingAtMarker,
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
        (chunk_map, chunks, players, entities, positions, rotations, mut looked_at_blocks): Self::SystemData,
    ) {
        for (_, entity, pos, rot) in (&players, &entities, &positions, &rotations).join() {
            if let Some(target) =
                chunk_map.ray_intersection(&chunks, pos.0, (rot.0 * Vector3::unit_x()) * 5.0)
            {
                if let Some(looked_at_block) = looked_at_blocks.get_mut(entity) {
                    *looked_at_block = target;
                } else {
                    looked_at_blocks.insert(entity, target).unwrap();
                }
            } else {
                looked_at_blocks.remove(entity);
            }
        }
    }
}
pub struct LookingAtMarkerSystem;
impl<'a> System<'a> for LookingAtMarkerSystem {
    type SystemData = (
        ReadStorage<'a, LookingAtMarker>,
        ReadStorage<'a, LookedAt>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (markers, looked_at_positions, mut positions): Self::SystemData) {
        for (looking_at_marker, position) in (&markers, &mut positions).join() {
            if let Some(target) = looked_at_positions.get(looking_at_marker.player) {
                position.0 = target.intersection;
            }
        }
    }
}
