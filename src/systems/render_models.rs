use crate::components::{Model, Position, Rotation, Scale};
use crate::manager::ModelManager;
use cgmath::{Matrix3, SquareMatrix, Vector3};
use finger_paint_wgpu::model::ModelMiddleWare;
use finger_paint_wgpu::{Transform, WgpuRenderer};
use specs::{Join, ReadStorage, System, Write, WriteStorage};

pub struct RenderModels<'a>(pub &'a mut WgpuRenderer, pub &'a mut ModelMiddleWare);
impl<'a> System<'a> for RenderModels<'_> {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'a, ModelManager>,
        WriteStorage<'a, Model>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Scale>,
        ReadStorage<'a, Rotation>,
    );

    fn run(&mut self, (mut manager, mut models, position, scale, rotation): Self::SystemData) {
        for model in manager.get_all_mut() {
            model.instances.clear();
        }
        for (model, position, scale, rotation) in (
            &mut models,
            &position,
            (&scale).maybe(),
            (&rotation).maybe(),
        )
            .join()
        {
            let t: Transform = Transform {
                position: position.0,
                rotation: rotation.map_or_else(Matrix3::identity, |m| m.0),
                scale: scale.map_or_else(|| Vector3::new(1.0, 1.0, 1.0), |r| r.0),
            };

            let instances = &mut manager.get(&model.0).unwrap().instances;
            instances.push(t);
        }
        let (device, queue) = self.0.device_and_queue();
        for model in manager.get_all_mut() {
            model.update(device, queue);
        }
    }
}
