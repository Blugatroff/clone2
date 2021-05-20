use crate::renderer::Renderer;
use specs::{ReadExpect, System};

pub struct UpdateCamera<'a>(pub &'a mut Renderer);
impl<'a> System<'a> for UpdateCamera<'_> {
    type SystemData = ReadExpect<'a, finger_paint_wgpu::Camera>;

    fn run(&mut self, camera: Self::SystemData) {
        let renderer = &mut self.0;
        *renderer.renderer.camera() = *camera;
    }
}
