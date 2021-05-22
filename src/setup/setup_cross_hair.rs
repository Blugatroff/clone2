use finger_paint_wgpu::WgpuRenderer;

use crate::flat_middleware::FlatMiddleWare;

use super::*;

pub fn setup_cross_hair(world: &mut World) -> Entity {
    let cross_hair = world
        .fetch_mut::<FlatMiddleWare>()
        .load_flat_mesh("res/cross_hair.png");
    let size = 0.0125;
    let aspect = world.fetch::<WgpuRenderer>().aspect();
    let cross_hair_position =
        Vector3::new(0.5, 0.5, 0.0) - Vector3::new(1.0, 1.0, 0.0) * size * 0.5;
    let cross_hair_size = Vector3::new(1.0, 1.0 * aspect, 1.0) * size;
    world
        .create_entity()
        .with(Position(cross_hair_position))
        .with(Scale(cross_hair_size))
        .with(FlatMesh(cross_hair))
        .build()
}
