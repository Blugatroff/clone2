use crate::components::{Position, Rotation, Scale, UvMesh};
use crate::manager::UvMeshManager;
use cgmath::{Matrix3, SquareMatrix, Vector3};
use finger_paint_wgpu::Transform;
use specs::{Join, ReadStorage, System, Write, WriteStorage};

pub struct RenderUvMeshes;
impl<'a> System<'a> for RenderUvMeshes {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Write<'a, UvMeshManager>,
        WriteStorage<'a, UvMesh>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Scale>,
        ReadStorage<'a, Rotation>,
    );

    fn run(&mut self, (mut manager, mut meshes, position, scale, rotation): Self::SystemData) {
        for mesh in manager.get_all_mut() {
            mesh.instances.clear();
        }
        for (model, position, scale, rotation) in (
            &mut meshes,
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
        for mesh in manager.get_all_mut() {
            mesh.update();
        }
    }
}
