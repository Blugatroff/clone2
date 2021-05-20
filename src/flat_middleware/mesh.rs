use crate::chunk_middle_ware::ChunkMesh;
use crate::flat_middleware::FlatVertex;
use bytemuck::{Pod, Zeroable};
use cgmath::Vector2;
use finger_paint_wgpu::wgpu::util::{BufferInitDescriptor, DeviceExt};
use finger_paint_wgpu::wgpu::{
    BindGroup, Buffer, BufferDescriptor, BufferUsage, ColorTargetState, CommandEncoder, Device,
    FilterMode, Queue, RenderPass, RenderPipeline, TextureView, VertexBufferLayout, VertexFormat,
};
use finger_paint_wgpu::{texture::Texture, MiddleWareConstructor};
use finger_paint_wgpu::{wgpu, MiddleWare};
use std::path::Path;
use std::sync::Arc;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub position: Vector2<f32>,
    pub size: Vector2<f32>,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct RawRect {
    pub position: [f32; 2],
    pub size: [f32; 2],
}
impl From<Rect> for RawRect {
    fn from(rect: Rect) -> Self {
        Self {
            position: rect.position.into(),
            size: rect.size.into(),
        }
    }
}
impl RawRect {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    shader_location: 2,
                    offset: 0,
                    format: VertexFormat::Float2,
                },
                wgpu::VertexAttribute {
                    shader_location: 3,
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    format: VertexFormat::Float2,
                },
            ],
        }
    }
}
pub struct FlatMesh {
    device: Arc<Device>,
    pub vertex_buffer: wgpu::Buffer,
    pub texture: Texture,
    pub bind_group: BindGroup,
    pub instances: Vec<Rect>,
    pub instance_buffer: Buffer,
}

impl FlatMesh {
    pub fn new<P: AsRef<Path>>(device: Arc<Device>, queue: &Queue, path: P) -> Self {
        let vertex_buffer = Self::create_vertex_buffer(
            &device,
            &[
                FlatVertex::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)),
                FlatVertex::new(Vector2::new(1.0, 1.0), Vector2::new(1.0, 1.0)),
                FlatVertex::new(Vector2::new(0.0, 1.0), Vector2::new(0.0, 1.0)),
                FlatVertex::new(Vector2::new(0.0, 0.0), Vector2::new(0.0, 0.0)),
                FlatVertex::new(Vector2::new(1.0, 0.0), Vector2::new(1.0, 0.0)),
                FlatVertex::new(Vector2::new(1.0, 1.0), Vector2::new(1.0, 1.0)),
            ],
        );
        let texture = Texture::load(
            &device,
            queue,
            path,
            FilterMode::Nearest,
            FilterMode::Nearest,
            false,
        )
        .unwrap();
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("diffuse bind group"),
            layout: &ChunkMesh::diffuse_bind_group_layout(&device),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });
        let instances = vec![];
        let instance_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("instance buffer"),
            size: 0,
            usage: BufferUsage::VERTEX,
            mapped_at_creation: false,
        });
        Self {
            device,
            vertex_buffer,
            texture,
            bind_group,
            instances,
            instance_buffer,
        }
    }
    pub fn update(&mut self) {
        let instances: Vec<RawRect> = self.instances.iter().map(|r| RawRect::from(*r)).collect();
        self.instance_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("instance buffer"),
            usage: BufferUsage::VERTEX,
            contents: bytemuck::cast_slice(&instances),
        });
    }
    fn create_vertex_buffer(device: &Device, vertices: &[FlatVertex]) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsage::VERTEX,
        })
    }
}

pub struct FlatMiddleWare {
    device: Arc<Device>,
    queue: Arc<Queue>,
    forward_pipeline: RenderPipeline,
    flat_meshes: Vec<Option<FlatMesh>>,
}

impl MiddleWareConstructor for FlatMiddleWare {
    fn new(renderer: &finger_paint_wgpu::WgpuRenderer) -> Self
    where
        Self: Sized,
    {
        let vs_shader = renderer.load_spirv(include_bytes!("vs.glsl.spv"));
        let fs_shader = renderer.load_spirv(include_bytes!("fs.glsl.spv"));

        let device = renderer.device();
        let queue = renderer.queue();

        let forward_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&format!("forward pipeline layout: {}", "chunk mesh")),
                bind_group_layouts: &[
                    &renderer.bind_group_layout(),
                    &ChunkMesh::diffuse_bind_group_layout(&device),
                ],
                push_constant_ranges: &[],
            });
        let forward_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("forward pipeline: {}", "flat mesh")),
            layout: Some(&forward_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_shader,
                entry_point: "main",
                buffers: &[FlatVertex::desc(), RawRect::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_shader,
                entry_point: "main",
                targets: &[ColorTargetState {
                    format: *renderer.format(),
                    alpha_blend: wgpu::BlendState {
                        src_factor: wgpu::BlendFactor::One,
                        dst_factor: wgpu::BlendFactor::Zero,
                        operation: wgpu::BlendOperation::Add,
                    },
                    color_blend: wgpu::BlendState {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                ..Default::default()
            },
            depth_stencil: renderer.depth_stencil().clone(),
            multisample: wgpu::MultisampleState::default(),
        });
        Self {
            device,
            queue,
            forward_pipeline,
            flat_meshes: vec![],
        }
    }
}
impl MiddleWare for FlatMiddleWare {
    fn name(&self) -> &str { "FlatMeshMiddleWare" }

    fn prepare(&self) {}

    fn render<'a>(&'a self, render_pass: &mut RenderPass<'a>) {
        render_flat_meshes(
            &self.forward_pipeline,
            render_pass,
            self.flat_meshes.iter().flatten(),
        );
    }

    fn encoder(&mut self, _device: &Device, _encoder: &mut CommandEncoder, _view: &TextureView) {}

    fn queue_submit(&mut self) {}
}

impl FlatMiddleWare {
    pub fn load_flat_mesh<P: AsRef<Path>>(
        &mut self,
        path: P,
    ) -> FlatMeshHandle {
        FlatMeshHandle::new({
            put_in_first_slot(&mut self.flat_meshes, FlatMesh::new(self.device.clone(), &self.queue, path))
        })
    }
    pub fn get_mut(&mut self, mesh: &FlatMeshHandle) -> Option<&mut FlatMesh> {
        self.flat_meshes[mesh.index].as_mut()
    }
    pub fn get_all(&mut self) -> &mut Vec<Option<FlatMesh>> {
        &mut self.flat_meshes
    }
}

#[derive(Clone)]
pub struct FlatMeshHandle {
    pub index: usize,
}
impl FlatMeshHandle {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

fn render_flat_mesh<'a, 'b>(pass: &'b mut wgpu::RenderPass<'a>, flat_mesh: &'a FlatMesh) {
    if !flat_mesh.instances.is_empty() {
        pass.set_vertex_buffer(0, flat_mesh.vertex_buffer.slice(..));
        pass.set_vertex_buffer(1, flat_mesh.instance_buffer.slice(..));
        pass.set_bind_group(1, &flat_mesh.bind_group, &[]);
        pass.draw(0..6, 0..flat_mesh.instances.len() as u32);
    }
}
fn render_flat_meshes<'a, 'b, U>(
    pipeline: &'a RenderPipeline,
    pass: &'b mut wgpu::RenderPass<'a>,
    mut chunk_meshes: U,
) where
    U: Iterator<Item = &'a FlatMesh>,
{
    if let Some(first) = chunk_meshes.next() {
        pass.push_debug_group("flat meshes");
        pass.set_pipeline(&pipeline);
        render_flat_mesh(pass, first);
        for chunk_mesh in chunk_meshes {
            render_flat_mesh(pass, chunk_mesh);
        }
        pass.pop_debug_group();
    }
}
pub fn put_in_first_slot<T>(vec: &mut Vec<Option<T>>, object: T) -> usize {
    for (i, o) in vec.iter_mut().enumerate() {
        if o.is_none() {
            *o = Some(object);
            return i;
        }
    }
    vec.push(Some(object));
    vec.len() - 1
}
