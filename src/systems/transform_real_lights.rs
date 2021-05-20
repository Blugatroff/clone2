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
        for (mut light, position, rotation) in (
            &mut lights.restrict_mut(),
            (&position).maybe(),
            (&rotation).maybe(),
        )
            .join()
        {
            if let Some(position) = position {
                if position.0
                    != light
                        .get_unchecked()
                        .real_light
                        .camera
                        .get_position()
                        .into()
                {
                    light
                        .get_mut_unchecked()
                        .real_light
                        .camera
                        .set_position(position.0);
                }
            }
            if let Some(rotation) = rotation {
                let direction = rotation.0 * Vector3::unit_x();
                if direction
                    != light
                        .get_unchecked()
                        .real_light
                        .camera
                        .get_direction()
                        .into()
                {
                    light
                        .get_mut_unchecked()
                        .real_light
                        .camera
                        .set_direction(direction);
                }
            }
        }
    }
}
