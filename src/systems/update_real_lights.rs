use crate::components::{Position, RealLight, Rotation};
use cgmath::Vector3;
use finger_paint_wgpu::{RealLightApi, WgpuRenderer};
use specs::{prelude::ComponentEvent, WriteExpect};
use specs::{BitSet, Join, ReadStorage, ReaderId, System, WriteStorage};

pub struct UpdateRealLights;
fn set_camera(light: &mut RealLight, position: Option<&Position>, rotation: Option<&Rotation>) {
    let mut a = false;
    let mut b = false;
    if let Some(position) = position {
        light.real_light.camera.set_position(position.0);
        a = true;
    }
    if let Some(rotation) = rotation {
        light
            .real_light
            .camera
            .set_direction(rotation.0 * Vector3::unit_x());
        b = true;
    }
    if a && b {
        light.real_light.enabled = true;
    } else {
        light.real_light.enabled = false;
    }
}
impl<'a> System<'a> for UpdateRealLights {
    type SystemData = (
        WriteStorage<'a, RealLight>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rotation>,
        WriteExpect<'a, WgpuRenderer>,
        WriteExpect<'a, ReaderId<ComponentEvent>>,
    );

    fn run(
        &mut self,
        (mut real_lights, positions, rotation, mut renderer, mut reader_id): Self::SystemData,
    ) {
        let mut created = BitSet::default();
        let mut modified = BitSet::default();
        let mut removed = BitSet::default();

        let created_events = real_lights.channel().read(&mut reader_id);
        for event in created_events {
            match event {
                ComponentEvent::Inserted(id) => created.add(*id),
                ComponentEvent::Modified(id) => modified.add(*id),
                ComponentEvent::Removed(id) => removed.add(*id),
            };
        }
        for (mut real_light, _, position, rotation) in (
            &mut real_lights,
            &created,
            (&positions).maybe(),
            (&rotation).maybe(),
        )
            .join()
        {
            set_camera(&mut real_light, position, rotation);
            let rlh = renderer.add_real_light(real_light.real_light.clone());
            real_light.rlh = Some(rlh);
        }
        for (mut real_light, _, position, rotation) in (
            &mut real_lights,
            &modified,
            (&positions).maybe(),
            (&rotation).maybe(),
        )
            .join()
        {
            set_camera(&mut real_light, position, rotation);
            real_light
                .rlh
                .as_mut()
                .unwrap()
                .set(real_light.real_light.clone())
        }
        for (mut real_light, _, position, rotation) in (
            &mut real_lights,
            &removed,
            (&positions).maybe(),
            (&rotation).maybe(),
        )
            .join()
        {
            set_camera(&mut real_light, position, rotation);
            real_light.rlh = None;
        }
        renderer.update_real_lights();
    }
}
