use finger_paint_wgpu::WgpuRenderer;
use specs::{ReadExpect, System, WriteExpect};

pub struct UpdateCamera;
impl<'a> System<'a> for UpdateCamera {
    type SystemData = (
        ReadExpect<'a, finger_paint_wgpu::Camera>,
        WriteExpect<'a, WgpuRenderer>,
    );

    fn run(&mut self, (mut camera, mut renderer): Self::SystemData) {
        let camera = &mut camera;
        let renderer = &mut renderer;
        *renderer.camera() = **camera;
    }
}
