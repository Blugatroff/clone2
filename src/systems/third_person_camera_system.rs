use crate::resources::DeltaTime;
use crate::{
    components::{Position, Rotation, ThirdPersonCamera},
    math_utils::mix,
};
use cgmath::Vector3;
use specs::{Entities, Join, Read, ReadStorage, System, WriteStorage};

pub struct ThirdPersonCameraSystem;
impl<'a> System<'a> for ThirdPersonCameraSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        Read<'a, DeltaTime>,
        ReadStorage<'a, ThirdPersonCamera>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Rotation>,
    );

    fn run(
        &mut self,
        (entities, delta_time, cameras, mut positions, mut rotations): Self::SystemData,
    ) {
        let dt = delta_time.0;
        for (entity, camera) in (&entities, &cameras).join() {
            let mut offset = Vector3::new(0.0, 0.0, 0.0);
            if let Some(Rotation(target_rotation)) = rotations.get(camera.0).copied() {
                if let Some(Rotation(camera_rotation)) = rotations.get_mut(entity) {
                    *camera_rotation = target_rotation;
                    offset = (target_rotation * Vector3::unit_x()) * -3.0;
                }
            }
            if let Some(Position(target_position)) = positions.get(camera.0).copied() {
                if let Some(Position(camera_position)) = positions.get_mut(entity) {
                    *camera_position = mix(*camera_position, target_position + offset, 5.0 * dt);
                }
            }
        }
    }
}
