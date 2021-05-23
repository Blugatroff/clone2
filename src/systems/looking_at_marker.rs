use specs::{Join, ReadStorage, System, WriteStorage};

use crate::components::{LookedAt, LookingAtMarker, Position};

pub struct LookingAtMarkerSystem;
impl<'a> System<'a> for LookingAtMarkerSystem {
    type SystemData = (
        ReadStorage<'a, LookingAtMarker>,
        ReadStorage<'a, LookedAt>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (markers, looked_at_positions, mut positions): Self::SystemData) {
        for (looking_at_marker, position) in (&markers, &mut positions).join() {
            if let Some(target) = looked_at_positions.get(looking_at_marker.player) {
                position.0 = target.intersection;
            }
        }
    }
}
