use crate::dir::Dir;
use crate::{blocks::Block, components::LookedAt};
use cgmath::{InnerSpace, Vector3};
use specs::{Component, ReadStorage};
use specs::{DenseVecStorage, Entity};
use std::collections::HashMap;

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
    pub fn all(&self) -> Vec<(Vector3<i32>, Entity)> {
        self.chunks.iter().map(|(k, v)| (*k, *v)).collect()
    }
    pub fn ray_intersection(
        &self,
        chunks: &ReadStorage<'_, Chunk>,
        ray_origin: Vector3<f32>,
        ray_vector: Vector3<f32>,
    ) -> Option<LookedAt> {
        let len: f32 = ray_vector.magnitude().ceil();
        let len: i32 = len as i32;

        let roi: Vector3<i32> = Self::f_coords_to_coords(ray_origin);
        let sx = roi.x - len;
        let ex = roi.x + len;
        let sy = roi.y - len;
        let ey = roi.y + len;
        let sz = roi.z - len;
        let ez = roi.z + len;

        let mut nearest: Option<LookedAt> = None;
        for x in sx..ex + 1 {
            for y in sy..ey + 1 {
                for z in sz..ez + 1 {
                    let current_coords = Vector3::new(x, y, z);
                    let (chunk_coords, block_coords) =
                        Self::coords_to_chunk_coords_and_block(current_coords);
                    if let Some(chunk) = self.get_chunk(chunk_coords) {
                        if let Some(chunk) = chunks.get(chunk) {
                            let block = chunk.get_block(block_coords);
                            if block != Block::Empty {
                                for (triangle, current_dir) in create_block_triangles().iter() {
                                    let triangle_pos: Vector3<f32> = Vector3::new(
                                        current_coords.x as f32,
                                        current_coords.y as f32,
                                        current_coords.z as f32,
                                    );
                                    if let Some(mut current_intersection) =
                                        ray_triangle_intersection(
                                            ray_origin - triangle_pos,
                                            ray_vector,
                                            *triangle,
                                        )
                                    {
                                        current_intersection += triangle_pos;
                                        if let Some(LookedAt {
                                            intersection,
                                            coords,
                                            dir,
                                        }) = &mut nearest
                                        {
                                            if (ray_origin - current_intersection).magnitude2()
                                                < (ray_origin - *intersection).magnitude2()
                                            {
                                                *intersection = current_intersection;
                                                *coords = current_coords;
                                                *dir = *current_dir;
                                            }
                                        } else {
                                            nearest = Some(LookedAt {
                                                intersection: current_intersection,
                                                coords: current_coords,
                                                dir: *current_dir,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        nearest
    }
}

const fn create_block_triangles() -> [(Triangle, Dir); 12] {
    [
        (
            Triangle {
                p0: Vector3::new(0.0, 0.0, 1.0),
                p1: Vector3::new(1.0, 0.0, 1.0),
                p2: Vector3::new(1.0, 1.0, 1.0),
            },
            Dir::North,
        ),
        (
            Triangle {
                p0: Vector3::new(0.0, 0.0, 1.0),
                p1: Vector3::new(1.0, 1.0, 1.0),
                p2: Vector3::new(0.0, 1.0, 1.0),
            },
            Dir::North,
        ),
        (
            Triangle {
                p0: Vector3::new(0.0, 0.0, 0.0),
                p1: Vector3::new(1.0, 1.0, 0.0),
                p2: Vector3::new(1.0, 0.0, 0.0),
            },
            Dir::South,
        ),
        (
            Triangle {
                p0: Vector3::new(0.0, 0.0, 0.0),
                p1: Vector3::new(0.0, 1.0, 0.0),
                p2: Vector3::new(1.0, 1.0, 0.0),
            },
            Dir::South,
        ),
        (
            Triangle {
                p0: Vector3::new(1.0, 0.0, 0.0),
                p1: Vector3::new(1.0, 1.0, 1.0),
                p2: Vector3::new(1.0, 0.0, 1.0),
            },
            Dir::East,
        ),
        (
            Triangle {
                p0: Vector3::new(1.0, 0.0, 0.0),
                p1: Vector3::new(1.0, 1.0, 0.0),
                p2: Vector3::new(1.0, 1.0, 1.0),
            },
            Dir::East,
        ),
        (
            Triangle {
                p0: Vector3::new(0.0, 0.0, 0.0),
                p1: Vector3::new(0.0, 0.0, 1.0),
                p2: Vector3::new(0.0, 1.0, 1.0),
            },
            Dir::West,
        ),
        (
            Triangle {
                p0: Vector3::new(0.0, 0.0, 0.0),
                p1: Vector3::new(0.0, 1.0, 1.0),
                p2: Vector3::new(0.0, 1.0, 0.0),
            },
            Dir::West,
        ),
        (
            Triangle {
                p0: Vector3::new(0.0, 1.0, 0.0),
                p1: Vector3::new(1.0, 1.0, 1.0),
                p2: Vector3::new(1.0, 1.0, 0.0),
            },
            Dir::Up,
        ),
        (
            Triangle {
                p0: Vector3::new(0.0, 1.0, 0.0),
                p1: Vector3::new(0.0, 1.0, 1.0),
                p2: Vector3::new(1.0, 1.0, 1.0),
            },
            Dir::Up,
        ),
        (
            Triangle {
                p0: Vector3::new(0.0, 0.0, 0.0),
                p1: Vector3::new(1.0, 0.0, 0.0),
                p2: Vector3::new(1.0, 0.0, 1.0),
            },
            Dir::Down,
        ),
        (
            Triangle {
                p0: Vector3::new(0.0, 0.0, 0.0),
                p1: Vector3::new(1.0, 0.0, 1.0),
                p2: Vector3::new(0.0, 0.0, 1.0),
            },
            Dir::Down,
        ),
    ]
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

#[derive(Debug, Clone)]
pub struct Neighbours<T> {
    pub north: Option<T>,
    pub south: Option<T>,
    pub west: Option<T>,
    pub east: Option<T>,
    pub up: Option<T>,
    pub down: Option<T>,
}
impl<'a, T> Neighbours<T> {
    pub fn iter(&'a self) -> NeighboursIter<'a, T> {
        NeighboursIter {
            current: 0,
            neighbours: &self,
        }
    }
    pub fn clear(&mut self) {
        self.north = None;
        self.south = None;
        self.west = None;
        self.east = None;
        self.up = None;
        self.down = None;
    }
    pub fn new() -> Self {
        Self {
            north: None,
            south: None,
            west: None,
            east: None,
            up: None,
            down: None,
        }
    }
}
pub struct NeighboursIter<'a, T> {
    current: u8,
    neighbours: &'a Neighbours<T>,
}
impl<'a, T> Iterator for NeighboursIter<'a, T> {
    type Item = (Dir, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.current += 1;
            match self.current - 1 {
                0 => {
                    if let Some(north) = &self.neighbours.north {
                        return Some((Dir::North, north));
                    }
                }
                1 => {
                    if let Some(south) = &self.neighbours.south {
                        return Some((Dir::South, south));
                    }
                }
                2 => {
                    if let Some(west) = &self.neighbours.west {
                        return Some((Dir::West, west));
                    }
                }
                3 => {
                    if let Some(east) = &self.neighbours.east {
                        return Some((Dir::East, east));
                    }
                }
                4 => {
                    if let Some(up) = &self.neighbours.up {
                        return Some((Dir::Up, up));
                    }
                }
                5 => {
                    if let Some(down) = &self.neighbours.down {
                        return Some((Dir::Down, down));
                    }
                }
                _ => return None,
            }
        }
    }
}

#[test]
fn neighbour_test() {
    let mut neighbours = Neighbours {
        north: Some(0),
        south: None,
        west: Some(45),
        east: Some(-24345),
        up: None,
        down: Some(349934),
    }
    .iter();
    assert_eq!(neighbours.next(), Some((Dir::North, &0)));
    assert_eq!(neighbours.next(), Some((Dir::West, &45)));
    assert_eq!(neighbours.next(), Some((Dir::East, &-24345)));
    assert_eq!(neighbours.next(), Some((Dir::Down, &349934)));
    assert_eq!(neighbours.next(), None);
}

pub const CHUNK_SIZE: usize = 16;
#[derive(Debug, Component)]
pub struct Chunk {
    pub position: Vector3<i32>,
    pub blocks: Box<[[[Block; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]>,
    /// whether the mesh of this chunk has to be regenerated
    pub regenerate_mesh: bool,
    pub update_neighbours: Neighbours<()>,
}

impl Chunk {
    pub fn empty(position: Vector3<i32>) -> Self {
        Self {
            position,
            blocks: Box::new([[[Block::Empty; CHUNK_SIZE]; CHUNK_SIZE]; CHUNK_SIZE]),
            regenerate_mesh: false,
            update_neighbours: Neighbours::new(),
        }
    }
    pub fn get_block(&self, pos: Vector3<u16>) -> Block {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let z = pos.z as usize;
        self.blocks[x][y][z]
    }
    pub fn set_block(&mut self, pos: Vector3<u16>, block: Block) {
        let x = pos.x as usize;
        let y = pos.y as usize;
        let z = pos.z as usize;
        let b = &mut self.blocks[x][y][z];
        if *b != block {
            *b = block;
            self.regenerate_mesh = true;
            if x == 0 {
                self.update_neighbours.west = Some(());
            } else if x == CHUNK_SIZE - 1 {
                self.update_neighbours.east = Some(());
            }
            if y == 0 {
                self.update_neighbours.down = Some(());
            } else if y == CHUNK_SIZE - 1 {
                self.update_neighbours.up = Some(());
            }
            if z == 0 {
                self.update_neighbours.south = Some(());
            } else if z == CHUNK_SIZE - 1 {
                self.update_neighbours.north = Some(());
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p0: Vector3<f32>,
    pub p1: Vector3<f32>,
    pub p2: Vector3<f32>,
}

pub fn ray_triangle_intersection(
    ray_origin: Vector3<f32>,
    ray_vector: Vector3<f32>,
    triangle: Triangle,
) -> Option<Vector3<f32>> {
    let vertex0 = triangle.p0;
    let vertex1 = triangle.p1;
    let vertex2 = triangle.p2;
    let edge1 = vertex1 - vertex0;
    let edge2 = vertex2 - vertex0;
    let ha = ray_vector.cross(edge2);
    let aa = edge1.dot(ha);
    if aa > -0.0000001 && aa < 0.0000001 {
        return None; // This ray is parallel to this triangle.
    }
    let fa = 1.0 / aa;
    let sa = ray_origin - vertex0;
    let ua = fa * sa.dot(ha);
    if !(0.0..=1.0).contains(&ua) {
        return None;
    }
    let qa = sa.cross(edge1);
    let va = fa * ray_vector.dot(qa);
    if va < 0.0 || ua + va > 1.0 {
        return None;
    }
    // At this stage we can compute t to find out where the intersection point is on the line.
    let ta = fa * edge2.dot(qa);
    if ta > 0.0000001
    // ray intersection
    {
        let out_intersection_point = ray_origin + ray_vector * ta;
        Some(out_intersection_point)
    } else {
        // This means that there is a line intersection but not a ray intersection.
        None
    }
}
