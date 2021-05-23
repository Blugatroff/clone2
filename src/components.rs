use crate::dir::Dir;
use crate::manager::{EcsModelHandle, EcsUvMesh};
use cgmath::{Matrix3, Vector3};
use specs::Component;
use specs::{DenseVecStorage, Entity};

#[derive(Component, Debug, Clone, Copy)]
pub struct Position(pub Vector3<f32>);

#[derive(Component, Debug, Clone, Copy)]
pub struct Scale(pub Vector3<f32>);

#[derive(Component, Debug, Clone, Copy)]
pub struct Rotation(pub Matrix3<f32>);

#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity(pub Vector3<f32>);

#[derive(Component)]
pub struct Model(pub EcsModelHandle);

#[derive(Component)]
pub struct UvMesh(pub EcsUvMesh);

#[derive(Component)]
pub struct FlatMesh(pub crate::flat_middleware::FlatMesh);

#[derive(Component, Debug)]
pub struct Camera {
    pub fov: f32,
}

pub struct ChunkMesh(pub crate::chunk_middle_ware::ChunkMesh);
impl Component for ChunkMesh {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Component, Debug)]
pub struct ThirdPersonCamera(pub Entity);

#[derive(Component, Debug)]
pub struct FirstPersonController {
    pub yaw: f32,
    pub pitch: f32,
    pub speed: f32,
    pub key_turn_speed: f32,
    pub boost: f32,
}

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component)]
pub struct RealLight(pub finger_paint_wgpu::RLH);

#[derive(Component, Debug)]
pub struct LookedAt {
    pub intersection: Vector3<f32>,
    pub coords: Vector3<i32>,
    pub dir: Dir,
}

#[derive(Component, Debug)]
pub struct LookingAtMarker {
    pub player: Entity,
}

#[derive(Component, Debug)]
pub struct BlockHighlightCube(pub Entity);

/// stores the Player so the Sun and more importantly its shadow can follow the Player
#[derive(Component, Debug)]
pub struct Sun {
    pub player: Entity,
    pub size: f32,
    pub distance: f32,
}
