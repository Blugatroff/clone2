use crate::components::{FlatMesh, Position, Scale};
use crate::flat_middleware::Rect;
use cgmath::Vector2;
use specs::{Entities, Join, ReadStorage, System, WriteStorage};

pub struct RenderFlatMeshes;
impl<'a> System<'a> for RenderFlatMeshes {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, FlatMesh>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Scale>,
    );

    fn run(&mut self, (entities, mut meshes, position, scale): Self::SystemData) {
        for (_, mesh) in (&entities, &mut meshes).join() {
            mesh.0.instances.clear();
        }
        for (model, position, scale) in (&mut meshes, &position, (&scale).maybe()).join() {
            model.0.instances.push(Rect {
                position: position.0.truncate(),
                size: if let Some(scale) = scale {
                    Vector2::new(1.0 * scale.0.x, 1.0 * scale.0.y)
                } else {
                    Vector2::new(1.0, 1.0)
                },
            })
        }
        for (_, mesh) in (&entities, &mut meshes).join() {
            mesh.0.update();
        }
    }
}
