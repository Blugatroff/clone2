use crate::chunk::CHUNK_SIZE;
use crate::chunk_middle_ware::ChunkVertex;
use bytemuck::{Pod, Zeroable};
use cgmath::Vector3;
use finger_paint_wgpu::texture::Texture;
use finger_paint_wgpu::wgpu::util::{BufferInitDescriptor, DeviceExt};
use finger_paint_wgpu::wgpu::{
    BindGroup, BindGroupLayout, Buffer, BufferDescriptor, BufferUsage, Device, Queue, RenderPass,
    RenderPipeline,
};
use finger_paint_wgpu::{wgpu, MiddleWare};
use std::path::Path;
use std::sync::Arc;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct ChunkUniform {
    position: [i32; 3],
    chunk_size: f32,
}

pub struct ChunkMesh {
    device: Arc<Device>,
    pub vertices: Vec<ChunkVertex>,
    pub vertex_buffer: wgpu::Buffer,
    pub uniform: ChunkUniform,
    pub uniform_buffer: wgpu::Buffer,
    pub bind_group: BindGroup,
    pub to_be_removed: bool,
    pub active: bool,
}

impl ChunkMesh {
    pub fn new(
        device: Arc<Device>,
        vertices: Vec<ChunkVertex>,
        position: Vector3<i32>,
        bind_group_layout: &BindGroupLayout,
        uv_buffer: &Buffer,
    ) -> Self {
        let vertex_buffer = Self::create_vertex_buffer(&device, &vertices);
        let uniform = ChunkUniform {
            position: position.into(),
            chunk_size: CHUNK_SIZE as f32,
        };
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Chunk Uniform Buffer"),
            contents: bytemuck::bytes_of(&uniform),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uv_buffer,
                        offset: 0,
                        size: None,
                    },
                },
            ],
            label: None,
        });
        Self {
            device,
            vertices,
            vertex_buffer,
            uniform,
            uniform_buffer,
            bind_group,
            to_be_removed: false,
            active: true,
        }
    }
    pub fn update_vertices(&mut self) {
        self.vertex_buffer = Self::create_vertex_buffer(&self.device, &self.vertices);
    }
    fn create_vertex_buffer(device: &Device, vertices: &[ChunkVertex]) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Cubes Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsage::VERTEX,
        })
    }
    pub fn bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniform layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<ChunkUniform>() as wgpu::BufferAddress,
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        })
    }
    pub fn diffuse_bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: true,
                        comparison: false,
                    },
                    count: None,
                },
            ],
            label: Some("texture_bind_group_layout"),
        })
    }
}

pub struct ChunkMeshMiddleWare {
    device: Arc<Device>,
    shadow_pipeline: RenderPipeline,
    forward_pipeline: RenderPipeline,
    bind_group_layout: BindGroupLayout,
    diffuse_bind_group_layout: BindGroupLayout,
    bind_group: Option<BindGroup>,
    uv_buffer: Buffer,
    texture: Option<Texture>,
}
impl ChunkMeshMiddleWare {
    pub fn prepare<'a, 'b, I>(&'a self, chunks: I) -> ChunkMeshMiddleWareRenderable<'a, 'b, I>
    where
        I: Iterator<Item = &'b ChunkMesh> + Clone,
    {
        ChunkMeshMiddleWareRenderable {
            inner: self,
            meshes: chunks,
        }
    }
}
pub struct ChunkMeshMiddleWareRenderable<'a, 'b, I>
where
    I: Iterator<Item = &'b ChunkMesh> + Clone,
{
    inner: &'a ChunkMeshMiddleWare,
    meshes: I,
}

impl<'a, 'b, I> MiddleWare for ChunkMeshMiddleWareRenderable<'a, 'b, I>
where
    I: Iterator<Item = &'b ChunkMesh> + Clone,
{
    fn name(&self) -> &str {
        "ChunkMeshMiddleWare"
    }

    fn render<'pass>(&'pass self, render_pass: &mut RenderPass<'pass>) {
        render_chunk_meshes(
            &self.inner.forward_pipeline,
            render_pass,
            self.inner.bind_group.as_ref().unwrap(),
            self.meshes.clone(),
            2,
        )
    }

    fn render_shadow_pass<'c>(&'c self, render_pass: &mut RenderPass<'c>) {
        render_chunk_meshes(
            &self.inner.shadow_pipeline,
            render_pass,
            self.inner.bind_group.as_ref().unwrap(),
            self.meshes.clone(),
            1,
        );
    }
}

impl finger_paint_wgpu::MiddleWareConstructor for ChunkMeshMiddleWare {
    fn new(renderer: &finger_paint_wgpu::WgpuRenderer) -> Self
    where
        Self: Sized,
    {
        let shadow_shader = renderer.load_spirv(include_bytes!("shadow.glsl.spv"));
        let vs_shader = renderer.load_spirv(include_bytes!("vs.glsl.spv"));
        let fs_shader = renderer.load_spirv(include_bytes!("fs.glsl.spv"));
        let device = renderer.device();
        let bind_group_layout = ChunkMesh::bind_group_layout(&device);
        let shadow_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&format!("shadow pipeline layout: {}", "chunk mesh")),
                bind_group_layouts: &[&renderer.shadow_bind_group_layout(), &bind_group_layout],
                push_constant_ranges: &[],
            });
        let shadow_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("chunk shadow pass pipeline"),
            layout: Some(&shadow_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shadow_shader,
                entry_point: "main",
                buffers: &[ChunkVertex::desc()],
            },
            fragment: None,
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                ..Default::default()
            },
            depth_stencil: renderer.shadow_depth_stencil().clone(),
            multisample: wgpu::MultisampleState::default(),
        });
        let diffuse_bind_group_layout = ChunkMesh::diffuse_bind_group_layout(&device);
        let forward_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(&format!("forward pipeline layout: {}", "chunk mesh")),
                bind_group_layouts: &[
                    &renderer.bind_group_layout(),
                    &diffuse_bind_group_layout,
                    &bind_group_layout,
                ],
                push_constant_ranges: &[],
            });
        let forward_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("forward pipeline: {}", "chunk mesh")),
            layout: Some(&forward_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_shader,
                entry_point: "main",
                buffers: &[ChunkVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_shader,
                entry_point: "main",
                targets: &[finger_paint_wgpu::wgpu::ColorTargetState::from(
                    *renderer.format(),
                )],
            }),
            primitive: wgpu::PrimitiveState {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                ..Default::default()
            },
            depth_stencil: renderer.depth_stencil().clone(),
            multisample: wgpu::MultisampleState::default(),
        });
        let uv_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("UvBuffer"),
            usage: wgpu::BufferUsage::STORAGE,
            size: 0,
            mapped_at_creation: false,
        });
        Self {
            device,
            shadow_pipeline,
            forward_pipeline,
            bind_group_layout,
            diffuse_bind_group_layout,
            bind_group: None,
            uv_buffer,
            texture: None,
        }
    }
}
impl ChunkMeshMiddleWare {
    pub fn load_atlas<P: AsRef<Path>>(&mut self, queue: &Queue, path: P) {
        self.texture = Some(
            Texture::load(
                &self.device,
                queue,
                path,
                wgpu::FilterMode::Nearest,
                wgpu::FilterMode::Nearest,
                false,
            )
            .unwrap(),
        );
        self.bind_group = Some(Self::create_bind_group(
            &self.diffuse_bind_group_layout,
            self.texture.as_ref().unwrap(),
            &self.device,
        ));
    }
    fn create_bind_group(
        diffuse_bind_group_layout: &BindGroupLayout,
        texture: &Texture,
        device: &Device,
    ) -> BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("diffuse bind group"),
            layout: diffuse_bind_group_layout,
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
        })
    }
    pub fn load_chunk_mesh(&self, vertices: Vec<ChunkVertex>, position: Vector3<i32>) -> ChunkMesh {
        ChunkMesh::new(
            self.device.clone(),
            vertices,
            position,
            &self.bind_group_layout,
            &self.uv_buffer,
        )
    }
    pub fn load_uvs(&mut self, uvs: &[[f32; 2]]) {
        self.uv_buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("UvBuffer"),
            contents: bytemuck::cast_slice(uvs),
            usage: wgpu::BufferUsage::STORAGE,
        });
        self.bind_group = Some(Self::create_bind_group(
            &self.diffuse_bind_group_layout,
            self.texture.as_ref().unwrap(),
            &self.device,
        ));
    }
}

fn render_chunk_mesh<'a, 'b>(
    pass: &'b mut wgpu::RenderPass<'a>,
    chunk_mesh: &'a ChunkMesh,
    bgi: u32,
) {
    pass.set_vertex_buffer(0, chunk_mesh.vertex_buffer.slice(..));
    pass.set_bind_group(bgi, &chunk_mesh.bind_group, &[]);
    pass.draw(0..chunk_mesh.vertices.len() as u32, 0..1);
}
fn render_chunk_meshes<'a, 'b, 'c, U>(
    pipeline: &'a RenderPipeline,
    pass: &'b mut wgpu::RenderPass<'a>,
    bind_group: &'a BindGroup,
    mut chunk_meshes: U,
    bgi: u32,
) where
    U: Iterator<Item = &'c ChunkMesh>,
    'c: 'a,
{
    if let Some(first) = chunk_meshes.next() {
        pass.set_pipeline(&pipeline);
        if bgi != 1 {
            pass.set_bind_group(1, bind_group, &[]);
        }
        render_chunk_mesh(pass, first, bgi);
        for chunk_mesh in chunk_meshes {
            render_chunk_mesh(pass, chunk_mesh, bgi);
        }
    }
}
