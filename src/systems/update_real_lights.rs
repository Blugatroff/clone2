use crate::components::{Position, RealLight, Rotation};
use crate::renderer::Renderer;
use cgmath::Vector3;
use finger_paint_wgpu::RealLightApi;
use specs::prelude::ComponentEvent;
use specs::{BitSet, Join, ReadStorage, ReaderId, System, WriteStorage};

pub struct UpdateRealLights<'a> {
    pub renderer: &'a mut Renderer,
    pub created: BitSet,
    pub modified: BitSet,
    pub removed: BitSet,
    pub reader_id: &'a mut ReaderId<ComponentEvent>,
}

impl<'a> UpdateRealLights<'a> {
    pub fn new(renderer: &'a mut Renderer, reader_id: &'a mut ReaderId<ComponentEvent>) -> Self {
        Self {
            renderer,
            created: BitSet::default(),
            modified: BitSet::default(),
            removed: BitSet::default(),
            reader_id,
        }
    }
}
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
impl<'a> System<'a> for UpdateRealLights<'_> {
    type SystemData = (
        WriteStorage<'a, RealLight>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rotation>,
    );

    fn run(&mut self, (mut real_lights, positions, rotation): Self::SystemData) {
        self.created.clear();
        self.modified.clear();
        self.removed.clear();

        let created_events = real_lights.channel().read(self.reader_id);
        for event in created_events {
            match event {
                ComponentEvent::Inserted(id) => self.created.add(*id),
                ComponentEvent::Modified(id) => self.modified.add(*id),
                ComponentEvent::Removed(id) => self.removed.add(*id),
            };
        }
        for (mut real_light, _, position, rotation) in (
            &mut real_lights,
            &self.created,
            (&positions).maybe(),
            (&rotation).maybe(),
        )
            .join()
        {
            set_camera(&mut real_light, position, rotation);
            let rlh = self
                .renderer
                .renderer
                .add_real_light(real_light.real_light.clone());
            real_light.rlh = Some(rlh);
        }
        for (mut real_light, _, position, rotation) in (
            &mut real_lights,
            &self.modified,
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
            &self.removed,
            (&positions).maybe(),
            (&rotation).maybe(),
        )
            .join()
        {
            set_camera(&mut real_light, position, rotation);
            real_light.rlh = None;
        }
        self.renderer.renderer.update_real_lights();
    }
}
