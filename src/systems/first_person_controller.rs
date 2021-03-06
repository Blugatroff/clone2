use crate::{components, resources::DeltaTime};
use crate::{
    components::{Position, Rotation},
    math_utils::rotation_matrix_from_direction,
};
use cgmath::{InnerSpace, Vector3};
use simple_winit::input::Input;
use simple_winit::input::Key::ScanCode;
use specs::{Join, Read, ReadExpect, System, WriteStorage};
use std::{
    f32::consts::PI,
    sync::{Arc, Mutex},
};

pub struct FirstPersonController;
impl<'a> System<'a> for FirstPersonController {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, components::FirstPersonController>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Rotation>,
        ReadExpect<'a, Arc<Mutex<Input>>>,
    );

    fn run(&mut self, (dt, mut controllers, mut position, mut rotation, input): Self::SystemData) {
        let input = input.lock().unwrap();
        let dt = dt.0;
        for (controller, position, rotation) in
            (&mut controllers, &mut position, &mut rotation).join()
        {
            let mut mouse_diff = input.mouse_diff();
            mouse_diff.0 *= -0.0025;
            mouse_diff.1 *= -0.0025;

            let speed = controller.speed
                * (input.key_held(ScanCode(42)) as i32 as f32 * controller.boost + 1.0);

            let look_right_left = (input.key_held(ScanCode(105)) as i32 as f32
                - input.key_held(ScanCode(106)) as i32 as f32)
                * controller.key_turn_speed
                * dt
                + mouse_diff.0;
            let look_up_down = (input.key_held(ScanCode(103)) as i32 as f32
                - input.key_held(ScanCode(108)) as i32 as f32)
                * controller.key_turn_speed
                * dt
                + mouse_diff.1;
            let forwards_backwards = (input.key_held(ScanCode(17)) as i32 as f32
                - input.key_held(ScanCode(31)) as i32 as f32)
                * speed
                * dt;
            let right_left = (input.key_held(ScanCode(32)) as i32 as f32
                - input.key_held(ScanCode(30)) as i32 as f32)
                * speed
                * dt;
            let up_down = (input.key_held(ScanCode(57)) as i32 as f32
                - input.key_held(ScanCode(29)) as i32 as f32)
                * speed
                * dt;

            controller.yaw += look_right_left;
            controller.pitch += look_up_down;

            controller.pitch = if controller.pitch < -PI / 2.0 + 0.005 {
                -PI / 2.0 + 0.005
            } else if controller.pitch > PI / 2.0 - 0.005 {
                PI / 2.0 - 0.005
            } else {
                controller.pitch
            };
            let direction = Vector3::new(
                controller.pitch.cos() * controller.yaw.sin(),
                controller.pitch.sin(),
                controller.pitch.cos() * controller.yaw.cos(),
            );

            let plane_direction = Vector3::new(direction.x, 0.0, direction.z).normalize();
            let right = Vector3::new(
                (controller.yaw - PI / 2.0).sin(),
                0.0,
                (controller.yaw - PI / 2.0).cos(),
            )
            .normalize();

            position.0 += plane_direction * forwards_backwards * controller.speed;
            position.0 += right * right_left * controller.speed;
            position.0 += Vector3::unit_y() * up_down * controller.speed;

            rotation.0 = rotation_matrix_from_direction(direction);
        }
    }
}
