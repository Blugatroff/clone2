use crate::blocks::Side;
use crate::blocks::{Atlas, Block};
use crate::chunk::{Chunk, ChunkMap, CHUNK_SIZE};
use crate::chunk_middle_ware::{ChunkMeshMiddleWare, ChunkVertex};
use crate::components::ChunkMesh;
use crate::dir::Dir;
use cgmath::Vector3;
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

pub struct ChunkMeshGeneration<'a>(pub &'a mut ChunkMeshMiddleWare, pub &'a mut Atlas);
impl<'a> System<'a> for ChunkMeshGeneration<'_> {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Chunk>,
        ReadStorage<'a, ChunkMesh>,
        Read<'a, ChunkMap>,
    );

    fn run(&mut self, (entities, mut chunks, mesh, chunk_map): Self::SystemData) {
        let mut counter = 6;
        for (entity, mesh) in (&entities, &mesh).join() {
            if let Some(chunk) = chunks.get(entity) {
                if chunk.regenerate_mesh {
                    //println!("generating mesh for chunk {:?}", chunk.position);
                    if counter <= 0 {
                        break;
                    }
                    counter -= 1;
                    let vertices = self.0.mesh_vertices(&mesh.0).unwrap();
                    vertices.clear();

                    let mut north = None;
                    if let Some(e) = chunk_map.get_chunk(chunk.position + Vector3::new(0, 0, 1)) {
                        if let Some(c) = chunks.get(e) {
                            north = Some(c);
                        }
                    }
                    let mut south = None;
                    if let Some(e) = chunk_map.get_chunk(chunk.position + Vector3::new(0, 0, -1)) {
                        if let Some(c) = chunks.get(e) {
                            south = Some(c);
                        }
                    }

                    let mut up = None;
                    if let Some(e) = chunk_map.get_chunk(chunk.position + Vector3::new(0, 1, 0)) {
                        if let Some(c) = chunks.get(e) {
                            up = Some(c);
                        }
                    }
                    let mut down = None;
                    if let Some(e) = chunk_map.get_chunk(chunk.position + Vector3::new(0, -1, 0)) {
                        if let Some(c) = chunks.get(e) {
                            down = Some(c);
                        }
                    }

                    let mut east = None;
                    if let Some(e) = chunk_map.get_chunk(chunk.position + Vector3::new(1, 0, 0)) {
                        if let Some(c) = chunks.get(e) {
                            east = Some(c);
                        }
                    }
                    let mut west = None;
                    if let Some(e) = chunk_map.get_chunk(chunk.position + Vector3::new(-1, 0, 0)) {
                        if let Some(c) = chunks.get(e) {
                            west = Some(c);
                        }
                    }

                    for x in 0..CHUNK_SIZE {
                        for y in 0..CHUNK_SIZE {
                            for z in 0..CHUNK_SIZE {
                                for dir in Dir::iter() {
                                    let mut discard =
                                        check_discard(dir, x as u16, y as u16, z as u16, &chunk);

                                    if dir == Dir::East {
                                        if let Some(east) = east {
                                            if x == CHUNK_SIZE - 1 {
                                                discard = east.blocks[0][y as usize][z as usize]
                                                    != Block::Empty;
                                            }
                                        }
                                    } else if dir == Dir::West {
                                        if let Some(west) = west {
                                            if x == 0 {
                                                discard = west.blocks[CHUNK_SIZE as usize - 1]
                                                    [y as usize]
                                                    [z as usize]
                                                    != Block::Empty;
                                            }
                                        }
                                    } else if dir == Dir::North {
                                        if let Some(north) = north {
                                            if z == CHUNK_SIZE - 1 {
                                                discard = north.blocks[x as usize][y as usize][0]
                                                    != Block::Empty;
                                            }
                                        }
                                    } else if dir == Dir::South {
                                        if let Some(south) = south {
                                            if z == 0 {
                                                discard = south.blocks[x as usize][y as usize]
                                                    [CHUNK_SIZE as usize - 1]
                                                    != Block::Empty;
                                            }
                                        }
                                    } else if dir == Dir::Up {
                                        if let Some(up) = up {
                                            if y == CHUNK_SIZE - 1 {
                                                discard = up.blocks[x as usize][0][z as usize]
                                                    != Block::Empty
                                            }
                                        }
                                    } else if dir == Dir::Down {
                                        if let Some(down) = down {
                                            if y == 0 {
                                                discard = down.blocks[x as usize]
                                                    [CHUNK_SIZE as usize - 1]
                                                    [z as usize]
                                                    != Block::Empty;
                                            }
                                        }
                                    }
                                    if !discard {
                                        add_face(
                                            self.1,
                                            chunk,
                                            [x as u8, y as u8, z as u8],
                                            dir,
                                            vertices,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    let mesh = self.0.get_mut(&mesh.0).unwrap();
                    mesh.update_vertices();
                    chunks.get_mut(entity).unwrap().regenerate_mesh = false;
                }
            }
        }
    }
}

fn check_discard(dir: Dir, x: u16, y: u16, z: u16, chunk: &Chunk) -> bool {
    match dir {
        Dir::North => {
            if z < CHUNK_SIZE as u16 - 1
                && chunk.blocks[x as usize][y as usize][z as usize + 1] != Block::Empty
            {
                return true;
            }
        }
        Dir::South => {
            if z > 0 && chunk.blocks[x as usize][y as usize][z as usize - 1] != Block::Empty {
                return true;
            }
        }
        Dir::East => {
            if x < CHUNK_SIZE as u16 - 1
                && chunk.blocks[x as usize + 1][y as usize][z as usize] != Block::Empty
            {
                return true;
            }
        }
        Dir::West => {
            if x > 0 && chunk.blocks[x as usize - 1][y as usize][z as usize] != Block::Empty {
                return true;
            }
        }
        Dir::Up => {
            if y < CHUNK_SIZE as u16 - 1
                && chunk.blocks[x as usize][y as usize + 1][z as usize] != Block::Empty
            {
                return true;
            }
        }
        Dir::Down => {
            if y > 0 && chunk.blocks[x as usize][y as usize - 1][z as usize] != Block::Empty {
                return true;
            }
        }
    }
    false
}

fn add_face(
    atlas: &mut Atlas,
    chunk: &Chunk,
    position: [u8; 3],
    face: Dir,
    vertices: &mut Vec<ChunkVertex>,
) {
    let mut add = |o, normal, uv_index| {
        vertices.push(ChunkVertex::new(
            Vector3::new(position[0], position[1], position[2]) + o,
            normal,
            uv_index,
        ))
    };

    let uv_index =
        chunk.blocks[position[0] as usize][position[1] as usize][position[2] as usize] as u16;

    let uv = if uv_index > 0 {
        /*Some(atlas.uvs_of_block(
            uv_index as usize,
            match face {
                Dir::North | Dir::South | Dir::East | Dir::West => Side::Side,
                Dir::Up => Side::Top,
                Dir::Down => Side::Base,
            },
        ))*/
        let indices = atlas.uvs_of_block_index(
            uv_index as usize,
            match face {
                Dir::North | Dir::South | Dir::East | Dir::West => Side::Side,
                Dir::Up => Side::Top,
                Dir::Down => Side::Base,
            },
        );
        //let mut uvs = [Vector2::new(0.0, 0.0); 4];
        let mut uvs = [0; 4];
        for i in 0..4 {
            //uvs[i] = atlas.all_uvs[indices[i] as usize].into();
            uvs[i] = indices[i] as u32;
        }
        Some(uvs)
    } else {
        None
    };

    if let Some(uv) = uv {
        #[rustfmt::skip]
        match face {
            Dir::North => {
                add(Vector3::new(0, 0, 1), Dir::North, uv[2]);
                add(Vector3::new(1, 0, 1), Dir::North, uv[3]);
                add(Vector3::new(1, 1, 1), Dir::North, uv[1]);

                add(Vector3::new(0, 0, 1), Dir::North, uv[2]);
                add(Vector3::new(1, 1, 1), Dir::North, uv[1]);
                add(Vector3::new(0, 1, 1), Dir::North, uv[0]);
            }
            Dir::South => {
                add(Vector3::new(0, 0, 0), Dir::South, uv[2]);
                add(Vector3::new(1, 1, 0), Dir::South, uv[1]);
                add(Vector3::new(1, 0, 0), Dir::South, uv[3]);

                add(Vector3::new(0, 0, 0), Dir::South, uv[2]);
                add(Vector3::new(0, 1, 0), Dir::South, uv[0]);
                add(Vector3::new(1, 1, 0), Dir::South, uv[1]);
            }
            Dir::East => {
                add(Vector3::new(1, 0, 0), Dir::East, uv[2]);
                add(Vector3::new(1, 1, 1), Dir::East, uv[1]);
                add(Vector3::new(1, 0, 1), Dir::East, uv[3]);

                add(Vector3::new(1, 0, 0), Dir::East, uv[2]);
                add(Vector3::new(1, 1, 0), Dir::East, uv[0]);
                add(Vector3::new(1, 1, 1), Dir::East, uv[1]);
            }
            Dir::West => {
                add(Vector3::new(0, 0, 0), Dir::West, uv[2]);
                add(Vector3::new(0, 0, 1), Dir::West, uv[3]);
                add(Vector3::new(0, 1, 1), Dir::West, uv[1]);

                add(Vector3::new(0, 0, 0), Dir::West, uv[2]);
                add(Vector3::new(0, 1, 1), Dir::West, uv[1]);
                add(Vector3::new(0, 1, 0), Dir::West, uv[0]);
            }
            Dir::Up => {
                add(Vector3::new(0, 1, 0), Dir::Up, uv[0]);
                add(Vector3::new(1, 1, 1), Dir::Up, uv[3]);
                add(Vector3::new(1, 1, 0), Dir::Up, uv[2]);

                add(Vector3::new(0, 1, 0), Dir::Up, uv[0]);
                add(Vector3::new(0, 1, 1), Dir::Up, uv[1]);
                add(Vector3::new(1, 1, 1), Dir::Up, uv[3]);
            }
            Dir::Down => {
                add(Vector3::new(0, 0, 0), Dir::Down, uv[0]);
                add(Vector3::new(1, 0, 0), Dir::Down, uv[2]);
                add(Vector3::new(1, 0, 1), Dir::Down, uv[3]);

                add(Vector3::new(0, 0, 0), Dir::Down, uv[0]);
                add(Vector3::new(1, 0, 1), Dir::Down, uv[3]);
                add(Vector3::new(0, 0, 1), Dir::Down, uv[1]);
            }
        }
    }
}
