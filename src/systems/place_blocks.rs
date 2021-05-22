use std::sync::{Arc, Mutex};

use crate::components::Player;
use crate::{blocks::Block, components::LookedAt};
use crate::{
    chunk::{Chunk, ChunkMap},
    chunk_middle_ware::ChunkMeshMiddleWare,
    components::ChunkMesh,
};
use cgmath::Vector3;
use simple_winit::input::Input;
use specs::{Entities, Join, ReadExpect, ReadStorage, System, Write, WriteStorage};

pub struct PlaceBlocks;

impl<'a> System<'a> for PlaceBlocks {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'a, ChunkMap>,
        WriteStorage<'a, Chunk>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, LookedAt>,
        ReadExpect<'a, Arc<Mutex<Input>>>,
        ReadExpect<'a, ton::Player>,
        Entities<'a>,
        WriteStorage<'a, ChunkMesh>,
        ReadExpect<'a, ChunkMeshMiddleWare>,
    );

    fn run(
        &mut self,
        (
            mut chunk_map,
            mut chunks,
            players,
            looked_at,
            input,
            player,
            entities,
            mut meshes,
            chunk_middleware,
        ): Self::SystemData,
    ) {
        let input = input.lock().unwrap();
        for (_, looked_at) in (&players, &looked_at).join() {
            if input.button_pressed(simple_winit::input::MouseButton::Right) {
                let dir: Vector3<i32> = looked_at.dir.into();
                let (chunk_coords, block_coords) =
                    ChunkMap::coords_to_chunk_coords_and_block(looked_at.coords + dir);
                if let Some(chunk) = chunk_map.get_chunk(chunk_coords) {
                    let chunk = chunks.get_mut(chunk).unwrap();
                    chunk.set_block(block_coords, Block::Stone);
                } else {
                    let mut chunk = Chunk::empty(chunk_coords);
                    chunk.set_block(block_coords, Block::Stone);
                    let mesh =
                        ChunkMesh(chunk_middleware.load_chunk_mesh(Vec::new(), chunk_coords));

                    let chunk = entities
                        .build_entity()
                        .with(chunk, &mut chunks)
                        .with(mesh, &mut meshes)
                        .build();
                    let sand = ton::PlayableSfxr::load_from_json(
                        &std::fs::read_to_string("./sand.json").unwrap(),
                    )
                    .unwrap();
                    player.play(sand).unwrap().detach();
                    chunk_map.set_chunk(chunk_coords, chunk);
                }
            }
        }
    }
}
