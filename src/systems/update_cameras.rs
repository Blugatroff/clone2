use crate::components::{Camera, Position, Rotation};
use cgmath::Vector3;
use specs::{Join, ReadStorage, System, WriteExpect};

pub struct UpdateCameras;
impl<'a> System<'a> for UpdateCameras {
    type SystemData = (
        WriteExpect<'a, finger_paint_wgpu::Camera>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rotation>,
    );

    fn run(&mut self, (mut the_camera, cameras, position, rotation): Self::SystemData) {
        for (camera, position, rotation) in (&cameras, &position, &rotation).join() {
            the_camera.set_direction(rotation.0 * Vector3::unit_x());
            the_camera.set_position(position.0);
            the_camera.set_fov(camera.fov);
        }
    }
}
