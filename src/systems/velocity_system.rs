use crate::components::{Position, Velocity};
use crate::resources::DeltaTime;
use specs::{Join, Read, ReadStorage, System, WriteStorage};

pub struct VelocitySystem;
impl<'a> System<'a> for VelocitySystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (delta_time, mut positions, velocities): Self::SystemData) {
        let dt = delta_time.0;
        for (position, velocity) in (&mut positions, &velocities).join() {
            position.0 += velocity.0 * dt;
        }
    }
}
