use crate::components::{Position, RealLight, Rotation};
use cgmath::Vector3;
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct TransformRealLights;
impl<'a> System<'a> for TransformRealLights {
    type SystemData = (
        WriteStorage<'a, RealLight>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rotation>,
    );

    fn run(&mut self, (mut lights, position, rotation): Self::SystemData) {
        for (light, position, rotation) in
            (&mut lights, (&position).maybe(), (&rotation).maybe()).join()
        {
            if let Some(position) = position {
                let mut l = light.0.get();
                l.camera.set_position(position.0);
                light.0.set(l);
            }
            if let Some(rotation) = rotation {
                let direction = rotation.0 * Vector3::unit_x();
                let mut l = light.0.get();
                l.camera.set_direction(direction);
                light.0.set(l);
            }
        }
    }
}
