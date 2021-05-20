use crate::blocks::Block;
use crate::chunk::{Chunk, ChunkMap, CHUNK_SIZE};
use crate::components::{ChunkMesh, Player, Position};
use crate::renderer::{Renderer};
use cgmath::{MetricSpace, Vector3};
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, Write};

pub struct GenerateChunks<'a>(pub &'a mut Renderer);
impl<'a> System<'a> for GenerateChunks<'_> {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        Write<'a, ChunkMap>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
    );

    fn run(&mut self, (entities, mut chunk_map, updater, player, positions): Self::SystemData) {
        let mut create_chunk = |position, chunk, map: &mut ChunkMap| {
            let new_chunk = entities.create();
            let mesh = self.0.chunk_mesh_middleware.load_chunk_mesh(Vec::new(), position);
            updater.insert(new_chunk, chunk);
            updater.insert(new_chunk, ChunkMesh(mesh));
            map.set_chunk(position, new_chunk);
        };

        for (_, position) in (&player, &positions).join() {
            let mut p = ChunkMap::f_coords_to_chunk_coords(position.0);
            p.y = 0;

            let range = 6;
            for x in -range..range + 1 {
                for z in -range..range + 1 {
                    let x = x + p.x;
                    let z = z + p.z;
                    let chunk_position = Vector3::new(x, 0, z);
                    if chunk_position.distance2(p) < range * range
                        && chunk_map.get_chunk(chunk_position).is_none()
                    {
                        println!("generating chunk: {:?}", chunk_position);
                        let mut chunk = Chunk::empty(chunk_position);
                        *chunk.blocks = [[[Block::Grass; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE];
                        chunk.regenerate_mesh = true;
                        chunk.update_neighbours.west = Some(());
                        chunk.update_neighbours.east = Some(());
                        chunk.update_neighbours.north = Some(());
                        chunk.update_neighbours.south = Some(());
                        chunk.update_neighbours.up = Some(());
                        chunk.update_neighbours.down = Some(());
                        create_chunk(chunk_position, chunk, &mut chunk_map);
                    }
                }
            }
        }
    }
}
