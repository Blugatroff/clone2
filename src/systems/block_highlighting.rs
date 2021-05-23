use cgmath::Vector3;
use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{BlockHighlightCube, LookedAt, Position};

pub struct BlockHighlighting;
impl<'a> System<'a> for BlockHighlighting {
    type SystemData = (
        ReadStorage<'a, LookedAt>,
        ReadStorage<'a, BlockHighlightCube>,
        WriteStorage<'a, Position>,
    );
    fn run(&mut self, (looked_at_positions, cubes, mut positions): Self::SystemData) {
        for (cube, position) in (&cubes, &mut positions).join() {
            if let Some(target) = looked_at_positions.get(cube.0) {
                position.0 = Vector3::new(
                    target.coords.x as f32,
                    target.coords.y as f32,
                    target.coords.z as f32,
                );
            }
        }
    }
}
