use crate::chunk::{Chunk, ChunkMap};
use crate::components::ChunkMesh;
use crate::dir::Dir;
use crate::renderer::{Renderer};
use cgmath::Vector3;
use specs::{Entities, Join, ReadStorage, System, Write, WriteStorage};

pub struct UpdateChunks<'a>(pub &'a mut Renderer);

impl<'a> System<'a> for UpdateChunks<'_> {
    type SystemData = (
        Write<'a, ChunkMap>,
        WriteStorage<'a, Chunk>,
        Entities<'a>,
        ReadStorage<'a, ChunkMesh>,
    );

    fn run(&mut self, (mut map, mut chunks, entities, chunk_meshes): Self::SystemData) {
        for chunk in self.0.chunk_mesh_middleware.get_all().iter_mut().flatten() {
            chunk.to_be_removed = true;
        }

        for (_, chunk, _) in (&entities, &chunk_meshes, &chunks).join() {
            self.0.chunk_mesh_middleware.get_mut(&chunk.0).unwrap().to_be_removed = false;
        }

        #[allow(clippy::manual_flatten)] // is it even possible with flatten?
        for c in self.0.chunk_mesh_middleware.get_all() {
            if let Some(chunk) = c {
                if chunk.to_be_removed {
                    for dir in Dir::iter() {
                        let d: Vector3<i32> = dir.into();
                        if let Some(e) = map.get_chunk(chunk.position() + d) {
                            if let Some(c) = chunks.get_mut(e) {
                                c.regenerate_mesh = true;
                            }
                        }
                    }
                    println!("removing chunk mesh: {:?}", chunk.position());
                    map.remove_chunk(chunk.position());
                    *c = None;
                }
            }
        }
    }
}
