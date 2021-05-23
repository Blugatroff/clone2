use cgmath::{InnerSpace, Vector3};
use specs::ReadStorage;

use crate::{blocks::Block, chunk::Chunk, chunk_map::ChunkMap, components::LookedAt, dir::Dir};

pub fn ray_chunks_intersection(
    chunk_map: &ChunkMap,
    chunks: &ReadStorage<'_, Chunk>,
    ray_origin: Vector3<f32>,
    ray_vector: Vector3<f32>,
) -> Option<LookedAt> {
    let len: f32 = ray_vector.magnitude().ceil();
    let len: i32 = len as i32;

    let roi: Vector3<i32> = ChunkMap::f_coords_to_coords(ray_origin);
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
                    ChunkMap::coords_to_chunk_coords_and_block(current_coords);
                if let Some(chunk) = chunk_map.get_chunk(chunk_coords) {
                    if let Some(chunk) = chunks.get(chunk) {
                        let block = chunk.get_block(block_coords);
                        if block != Block::Empty {
                            for (triangle, current_dir) in create_block_triangles().iter() {
                                let triangle_pos: Vector3<f32> = Vector3::new(
                                    current_coords.x as f32,
                                    current_coords.y as f32,
                                    current_coords.z as f32,
                                );
                                if let Some(mut current_intersection) = ray_triangle_intersection(
                                    ray_origin - triangle_pos,
                                    ray_vector,
                                    *triangle,
                                ) {
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
