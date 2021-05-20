use bytemuck::{Pod, Zeroable};
use cgmath::Vector2;
use finger_paint_wgpu::wgpu;
use finger_paint_wgpu::wgpu::{VertexBufferLayout, VertexFormat};
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable, Debug)]
pub struct FlatVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
impl FlatVertex {
    pub fn desc<'a>() -> VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    shader_location: 0,
                    offset: 0,
                    format: VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    shader_location: 1,
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    format: VertexFormat::Float2,
                },
            ],
        }
    }
    pub fn new(position: Vector2<f32>, uv: Vector2<f32>) -> Self {
        FlatVertex {
            position: position.into(),
            tex_coords: uv.into(),
        }
    }
}
