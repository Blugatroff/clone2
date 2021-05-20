use crate::chunk_middle_ware::ChunkMeshMiddleWare;
use crate::flat_middleware::FlatMiddleWare;

use finger_paint_wgpu::{lines::Lines, model::ModelMiddleWare, text::TextMiddleWare, WgpuRenderer};

use finger_paint_wgpu::uv_mesh::UvMeshMiddleWare;
use simple_winit::winit::window::Window;

pub struct Renderer {
    pub renderer: WgpuRenderer,
    pub text_middleware: TextMiddleWare,
    pub chunk_mesh_middleware: ChunkMeshMiddleWare,
    pub flat_middleware: FlatMiddleWare,
    pub uv_mesh_middleware: UvMeshMiddleWare,
    pub model_middleware: ModelMiddleWare,
    pub lines: Lines,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        let renderer = WgpuRenderer::new(window, false);
        let chunk_mesh_middleware = renderer.load_middle_ware::<ChunkMeshMiddleWare>();
        let text_middleware = renderer.load_middle_ware::<TextMiddleWare>();
        let flat_middleware = renderer.load_middle_ware::<FlatMiddleWare>();
        let uv_mesh_middleware = renderer.load_middle_ware::<UvMeshMiddleWare>();
        let lines = renderer.load_middle_ware::<Lines>();
        let model_middleware = renderer.load_middle_ware::<ModelMiddleWare>();
        Self {
            renderer,
            text_middleware,
            chunk_mesh_middleware,
            flat_middleware,
            uv_mesh_middleware,
            model_middleware,
            lines,
        }
    }
    pub fn render(&mut self, window: &Window, world: &specs::World) {
        let uv_mesh_manager = world.fetch_mut::<crate::manager::UvMeshManager>();
        let uv_meshes = uv_mesh_manager.get_all();
        let model_manager = world.fetch_mut::<crate::manager::ModelManager>();
        let models = model_manager.get_all();
        self.renderer.render(
            window,
            &mut [
                &mut self.lines,
                &mut self.chunk_mesh_middleware,
                &mut self.text_middleware,
                &mut self.uv_mesh_middleware.prepare(uv_meshes),
                &mut self.model_middleware.prepare(models),
                &mut self.flat_middleware,
            ],
        );
    }
}
