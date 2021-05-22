mod setup_cross_hair;
mod setup_highlight_cube;
mod setup_player;

use crate::components::{
    Camera, ChunkMesh, FirstPersonController, FlatMesh, LookingAtMarker, Model, Player, Position,
    RealLight, Rotation, Scale, ThirdPersonCamera, UvMesh, Velocity,
};
use crate::manager::{ModelManager, UvMeshManager};
use crate::resources::DeltaTime;
use crate::{
    chunk::{Chunk, ChunkMap, CHUNK_SIZE},
    components::LookedAt,
};
use cgmath::Vector3;
use finger_paint_wgpu::ViewMatrixMode;
use specs::Builder;
use specs::Entity;
use specs::{World, WorldExt};
use std::f32::consts::PI;

pub use setup_cross_hair::setup_cross_hair;
pub use setup_highlight_cube::setup_highlight_cube;
pub use setup_player::setup_player;

pub fn setup(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Scale>();
    world.register::<Rotation>();
    world.register::<Model>();
    world.register::<UvMesh>();
    world.register::<Camera>();
    world.register::<FirstPersonController>();
    world.register::<RealLight>();
    world.register::<ThirdPersonCamera>();
    world.register::<Chunk>();
    world.register::<ChunkMesh>();
    world.register::<Player>();
    world.register::<FlatMesh>();
    world.register::<LookedAt>();
    world.register::<LookingAtMarker>();
    world.insert(UvMeshManager::default());
    world.insert(ModelManager::default());
    world.insert(DeltaTime(0.0));
    world.insert(finger_paint_wgpu::Camera::new(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        ViewMatrixMode::Perspective {
            near: 0.05,
            far: CHUNK_SIZE as f32 * 16.0,
            fov: PI / 2.0,
            aspect: 1.0,
        },
    ));
    world.insert(ChunkMap::new());
}
