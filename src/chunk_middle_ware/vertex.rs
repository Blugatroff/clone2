use crate::dir::Dir;
use bytemuck::{Pod, Zeroable};
use cgmath::Vector3;
use finger_paint_wgpu::wgpu;
use finger_paint_wgpu::wgpu::{VertexBufferLayout, VertexFormat};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct ChunkVertex {
    v: u32,
}
impl ChunkVertex {
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                shader_location: 0,
                offset: 0,
                format: VertexFormat::Int,
            }],
        }
    }
    pub fn new(position: Vector3<u8>, normal: Dir, uv_index: u32) -> Self {
        ChunkVertex {
            v: pack(position, normal, uv_index),
        }
    }
}

fn pack(position: Vector3<u8>, normal: Dir, uv_index: u32) -> u32 {
    let normal = normal as u32;
    let x = position.x as u32;
    let y = position.y as u32;
    let z = position.z as u32;
    let mut v: u32 = 0;
    v |= uv_index;
    v |= normal << 11;
    v |= z << 14;
    v |= y << 20;
    v |= x << 26;
    v
}
#[allow(dead_code)]
#[rustfmt::skip]
fn unpack(v: u32) -> (Vector3<u8>, Dir, u32) {
    let uv_index =  v & 0b00000000000000000000011111111111;
    let dir      = (v & 0b00000000000000000011100000000000) >> 11;
    let z        = (v & 0b00000000000011111100000000000000) >> 14;
    let y        = (v & 0b00000011111100000000000000000000) >> 20;
    let x        = (v & 0b11111100000000000000000000000000) >> 26;

    (
        Vector3::new(x as u8, y as u8, z as u8),
        Dir::from(dir as u8),
        uv_index,
    )
}

#[test]
fn vertex_packing() {
    let uv_index = 435;
    let dir = Dir::West;
    let p = Vector3::new(9, 0, 16);
    let v = pack(p, dir, uv_index);
    assert_eq!(unpack(v), (p, dir, uv_index));
}
