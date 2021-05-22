use crate::manager::{EcsModelHandle, EcsUvMesh};
use cgmath::{Matrix3, Vector3};
use finger_paint_wgpu::{LightAttenuation, ViewMatrixMode};
use specs::{Component, FlaggedStorage};
use specs::{DenseVecStorage, Entity};
use crate::dir::Dir;

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

pub struct RealLight {
    pub real_light: finger_paint_wgpu::RealLight,
    pub rlh: Option<finger_paint_wgpu::RLH>,
}

#[allow(dead_code)]
impl RealLight {
    pub fn new(
        color: [f32; 4],
        default: f32,
        attenuation: LightAttenuation,
        mode: ViewMatrixMode,
        up: Vector3<f32>,
    ) -> Self {
        Self {
            real_light: finger_paint_wgpu::RealLight {
                camera: finger_paint_wgpu::Camera::new(
                    Vector3::new(0.0, 0.0, -1.0),
                    Vector3::new(0.0, 0.0, 0.0),
                    up,
                    mode,
                ),
                color,
                default,
                attenuation,
                enabled: false,
            },
            rlh: None,
        }
    }
}
impl Component for RealLight {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

#[derive(Component, Debug)]
pub struct LookedAt {
    pub intersection: Vector3<f32>,
    pub coords: Vector3<i32>,
    pub dir: Dir
}

#[derive(Component, Debug)]
pub struct LookingAtMarker {
    pub player: Entity,
}