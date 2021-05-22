use super::*;
use crate::dir::Dir;
use cgmath::{Matrix3, SquareMatrix, Vector2};
use finger_paint_wgpu::{
    texture::Texture,
    uv_mesh::{UvMeshMiddleWare, UvVertex},
    wgpu::FilterMode,
    WgpuRenderer,
};

pub fn setup_highlight_cube(world: &mut World, uv_mesh_middleware: &mut UvMeshMiddleWare) {
    let mut vertices = vec![];
    let mut add = |position: Vector3<i32>, dir: Dir, uv| {
        let position = Vector3::new(position.x as f32, position.y as f32, position.z as f32) * 1.1
            - Vector3::new(0.05, 0.05, 0.05);
        let normal: Vector3<i32> = dir.into();
        let normal = Vector3::new(normal.x as f32, normal.y as f32, normal.z as f32);
        vertices.push(UvVertex::new(position, normal, uv))
    };
    add(Vector3::new(0, 0, 1), Dir::North, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 0, 1), Dir::North, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 1, 1), Dir::North, Vector2::new(0.0, 0.0));

    add(Vector3::new(0, 0, 1), Dir::North, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 1, 1), Dir::North, Vector2::new(0.0, 0.0));
    add(Vector3::new(0, 1, 1), Dir::North, Vector2::new(0.0, 0.0));

    add(Vector3::new(0, 0, 0), Dir::South, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 1, 0), Dir::South, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 0, 0), Dir::South, Vector2::new(0.0, 0.0));

    add(Vector3::new(0, 0, 0), Dir::South, Vector2::new(0.0, 0.0));
    add(Vector3::new(0, 1, 0), Dir::South, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 1, 0), Dir::South, Vector2::new(0.0, 0.0));

    add(Vector3::new(1, 0, 0), Dir::East, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 1, 1), Dir::East, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 0, 1), Dir::East, Vector2::new(0.0, 0.0));

    add(Vector3::new(1, 0, 0), Dir::East, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 1, 0), Dir::East, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 1, 1), Dir::East, Vector2::new(0.0, 0.0));

    add(Vector3::new(0, 0, 0), Dir::West, Vector2::new(0.0, 0.0));
    add(Vector3::new(0, 0, 1), Dir::West, Vector2::new(0.0, 0.0));
    add(Vector3::new(0, 1, 1), Dir::West, Vector2::new(0.0, 0.0));

    add(Vector3::new(0, 0, 0), Dir::West, Vector2::new(0.0, 0.0));
    add(Vector3::new(0, 1, 1), Dir::West, Vector2::new(0.0, 0.0));
    add(Vector3::new(0, 1, 0), Dir::West, Vector2::new(0.0, 0.0));

    add(Vector3::new(0, 1, 0), Dir::Up, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 1, 1), Dir::Up, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 1, 0), Dir::Up, Vector2::new(0.0, 0.0));

    add(Vector3::new(0, 1, 0), Dir::Up, Vector2::new(0.0, 0.0));
    add(Vector3::new(0, 1, 1), Dir::Up, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 1, 1), Dir::Up, Vector2::new(0.0, 0.0));

    add(Vector3::new(0, 0, 0), Dir::Down, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 0, 0), Dir::Down, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 0, 1), Dir::Down, Vector2::new(0.0, 0.0));

    add(Vector3::new(0, 0, 0), Dir::Down, Vector2::new(0.0, 0.0));
    add(Vector3::new(1, 0, 1), Dir::Down, Vector2::new(0.0, 0.0));
    add(Vector3::new(0, 0, 1), Dir::Down, Vector2::new(0.0, 0.0));

    let data = [0, 0, 0, 192, 0, 0, 0, 192, 0, 0, 0, 192, 0, 0, 0, 192];

    let texture = {
        let renderer = world.fetch::<WgpuRenderer>();
        let (device, queue) = renderer.device_and_queue();
        Texture::from_raw(
            device,
            queue,
            (2, 2),
            &data,
            FilterMode::Nearest,
            FilterMode::Nearest,
            false,
        )
    };

    let mut uv_mesh = uv_mesh_middleware.create_uv_mesh(vertices, None, &texture);
    uv_mesh.transparent = true;
    let uv_mesh = world.fetch_mut::<UvMeshManager>().insert(uv_mesh);

    world
        .create_entity()
        .with(UvMesh(uv_mesh))
        .with(Position(Vector3::new(0.0, 0.0, 0.0)))
        .with(Rotation(Matrix3::identity()))
        .build();
}
