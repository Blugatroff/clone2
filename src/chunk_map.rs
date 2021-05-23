use std::collections::HashMap;

use cgmath::Vector3;
use specs::{Entity, ReadStorage};

use crate::{
    chunk::{Chunk, CHUNK_SIZE},
    components::LookedAt,
    ray_intersection::ray_chunks_intersection,
};

#[derive(Debug)]
pub struct ChunkMap {
    chunks: HashMap<Vector3<i32>, Entity>,
}

impl Default for ChunkMap {
    fn default() -> Self {
        Self {
            chunks: HashMap::default(),
        }
    }
}

impl ChunkMap {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }
    pub fn get_chunk(&self, pos: Vector3<i32>) -> Option<Entity> {
        self.chunks.get(&pos).copied()
    }
    pub fn get_chunk_from_coords(&self, coords: Vector3<i32>) -> Option<Entity> {
        self.get_chunk(Self::coords_to_chunk_coords(coords))
    }
    pub fn set_chunk(&mut self, pos: Vector3<i32>, chunk: Entity) {
        self.chunks.insert(pos, chunk);
    }
    pub fn remove_chunk(&mut self, pos: Vector3<i32>) {
        self.chunks.remove(&pos);
    }
    pub fn f_coords_to_coords(coords: Vector3<f32>) -> Vector3<i32> {
        let mut i_coords = Vector3::new(coords.x as i32, coords.y as i32, coords.z as i32);
        if coords.x < 0.0 {
            i_coords.x -= 1;
        }
        if coords.y < 0.0 {
            i_coords.y -= 1;
        }
        if coords.z < 0.0 {
            i_coords.z -= 1;
        }
        i_coords
    }
    pub fn f_coords_to_chunk_coords(f_coords: Vector3<f32>) -> Vector3<i32> {
        Self::coords_to_chunk_coords(Self::f_coords_to_coords(f_coords))
    }
    pub fn coords_to_chunk_coords(coords: Vector3<i32>) -> Vector3<i32> {
        let cs = CHUNK_SIZE as i32;
        let mut cc = coords / cs;
        if coords.x < 0 {
            cc.x -= 1
        }
        if coords.y < 0 {
            cc.y -= 1
        }
        if coords.z < 0 {
            cc.z -= 1
        }
        cc
    }
    pub fn coords_to_chunk_coords_and_block(coords: Vector3<i32>) -> (Vector3<i32>, Vector3<u16>) {
        let mut chunk_coords: Vector3<i32> = coords / CHUNK_SIZE as i32;
        if coords.x < 0 && coords.x % CHUNK_SIZE as i32 != 0 {
            chunk_coords.x -= 1;
        }
        if coords.y < 0 && coords.y % CHUNK_SIZE as i32 != 0 {
            chunk_coords.y -= 1;
        }
        if coords.z < 0 && coords.z % CHUNK_SIZE as i32 != 0 {
            chunk_coords.z -= 1;
        }
        let block_coord: Vector3<i32> = coords % 16;

        let block_coord: Vector3<u16> = Vector3::new(
            if block_coord.x < 0 {
                (CHUNK_SIZE as i32 + block_coord.x) as u16
            } else {
                block_coord.x as u16
            },
            if block_coord.y < 0 {
                (CHUNK_SIZE as i32 + block_coord.y) as u16
            } else {
                block_coord.y as u16
            },
            if block_coord.z < 0 {
                (CHUNK_SIZE as i32 + block_coord.z) as u16
            } else {
                block_coord.z as u16
            },
        );
        (chunk_coords, block_coord)
    }
    pub fn ray_intersection(
        &self,
        chunks: &ReadStorage<'_, Chunk>,
        ray_origin: Vector3<f32>,
        ray_vector: Vector3<f32>,
    ) -> Option<LookedAt> {
        ray_chunks_intersection(self, chunks, ray_origin, ray_vector)
    }
}

#[test]
fn coords_to_chunk_position_and_block() {
    assert_eq!(
        ChunkMap::coords_to_chunk_coords_and_block(Vector3::new(10, 40, 20)),
        (Vector3::new(0, 2, 1), Vector3::new(10, 8, 4))
    );
    assert_eq!(
        ChunkMap::coords_to_chunk_coords_and_block(Vector3::new(10, 40, -20)),
        (Vector3::new(0, 2, -2), Vector3::new(10, 8, 12))
    );
    assert_eq!(
        ChunkMap::coords_to_chunk_coords_and_block(Vector3::new(-0, -5, -1)),
        (Vector3::new(0, -1, -1), Vector3::new(0, 11, 15))
    );
    assert_eq!(
        ChunkMap::coords_to_chunk_coords_and_block(Vector3::new(-16, -15, -1)),
        (Vector3::new(-1, -1, -1), Vector3::new(0, 1, 15))
    );
}
#[test]
fn f_coords_to_coords() {
    assert_eq!(
        ChunkMap::f_coords_to_coords(Vector3::new(10.5, 40.3, -20.3)),
        Vector3::new(10, 40, -21)
    );
    assert_eq!(
        ChunkMap::f_coords_to_coords(Vector3::new(-10.5, 0.0, -0.0)),
        Vector3::new(-11, 0, 0)
    );
}
#[test]
fn coords_to_chunk_coords() {
    assert_eq!(
        ChunkMap::coords_to_chunk_coords(Vector3::new(0, 0, 0)),
        Vector3::new(0, 0, 0)
    );
    assert_eq!(
        ChunkMap::coords_to_chunk_coords(Vector3::new(17, 0, -5)),
        Vector3::new(1, 0, -1)
    );
}
