use crate::components::{FlatMesh, Position, Scale};
use crate::flat_middleware::{FlatMiddleWare, Rect};
use cgmath::Vector2;
use specs::{Join, ReadStorage, System, WriteStorage};

pub struct RenderFlatMeshes<'a>(pub &'a mut FlatMiddleWare);
impl<'a> System<'a> for RenderFlatMeshes<'_> {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, FlatMesh>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Scale>,
    );

    fn run(&mut self, (mut meshes, position, scale): Self::SystemData) {
        for mesh in self.0.get_all().iter_mut().flatten() {
            mesh.instances.clear();
        }
        for (model, position, scale) in (&mut meshes, &position, (&scale).maybe()).join() {
            self.0.get_mut(&model.0).unwrap().instances.push(Rect {
                position: position.0.truncate(),
                size: if let Some(scale) = scale {
                    Vector2::new(1.0 * scale.0.x, 1.0 * scale.0.y)
                } else {
                    Vector2::new(1.0, 1.0)
                },
            })
        }
        self.0
            .get_all()
            .iter_mut()
            .flatten()
            .for_each(|m| m.update());
    }
}
