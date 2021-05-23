use std::f32::consts::PI;

use crate::{
    components::{Position, RealLight, Rotation, Sun},
    math_utils::rotation_matrix_from_direction,
    resources::Time,
};
use cgmath::{InnerSpace, Vector3};
use finger_paint_wgpu::{LightAttenuation, WgpuRenderer};
use specs::{Entities, Join, Read, ReadStorage, System, WriteExpect, WriteStorage};

pub struct SunSystem;
impl<'a> System<'a> for SunSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, RealLight>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Rotation>,
        ReadStorage<'a, Sun>,
        WriteExpect<'a, WgpuRenderer>,
        Read<'a, Time>,
        Entities<'a>,
    );

    fn run(
        &mut self,
        (mut lights, mut positions, mut rotation, suns, mut renderer, time, entities): Self::SystemData,
    ) {
        for (entity, light, rotation, sun) in (&entities, &mut lights, &mut rotation, &suns).join()
        {
            let t = time.day_time;
            // t: 0.0 -> midnight
            // t: 0.25 -> dawn
            // t: 0.5 -> noon
            // t: 0.75 -> dusk
            let sun_angle = t * PI * 2.0;
            let direction = Vector3::new(0.3, sun_angle.cos(), sun_angle.sin()).normalize();
            rotation.0 = rotation_matrix_from_direction(direction);
            let mut l = light.0.get();
            let distance_to_noon = (0.5 - t).min(t - 0.5);
            let distance_to_dusk_or_dawn = (0.25 - t).min(t - 0.75);

            let power = 1.0 * (1.0 - distance_to_dusk_or_dawn * 2.0);
            let red = 0.5 - distance_to_dusk_or_dawn;
            let blue = 1.0 - distance_to_noon;
            let green = blue * 0.75;
            l.color = [red, green, blue, power];
            if distance_to_noon > 0.4 {
                l.color = [0.0, 0.0, 0.0, 1.0]
            }
            renderer.clear_color(l.color);

            if let Some(player_position) = positions.get(sun.player).copied() {
                if let Some(light_position) = positions.get_mut(entity) {
                    light_position.0 = player_position.0 - direction * sun.distance;
                }
            }
            l.default = 1.0;
            l.camera
                .set_matrix_mode(finger_paint_wgpu::ViewMatrixMode::Orthographic {
                    near: 0.1,
                    far: sun.distance * 2.0,
                    left: -sun.size / 2.0,
                    right: sun.size / 2.0,
                    bottom: -sun.size / 2.0,
                    top: sun.size / 2.0,
                });
            l.attenuation = LightAttenuation {
                constant: 1.0,
                linear: 0.0,
                quadratic: 0.0,
            };
            light.0.set(l);
        }
    }
}
