use super::*;
use crate::{
    components::{self, LookingAtMarker, Model},
    manager::EcsModelHandle,
};
use cgmath::{Matrix3, SquareMatrix};

pub fn setup_player(world: &mut World, sphere_model: EcsModelHandle) {
    let player = world
        .create_entity()
        .with(Position(Vector3::new(0.0, 0.0, 0.0)))
        .with(Rotation(Matrix3::identity()))
        .with(FirstPersonController {
            yaw: 0.0,
            pitch: 0.0,
            speed: 2.5,
            key_turn_speed: 5.0,
            boost: 3.0,
        })
        .with(components::Camera { fov: PI / 2.0 })
        .with(Player)
        .build();
    world
        .create_entity()
        .with(Position(Vector3::new(0.0, 0.0, 0.0)))
        .with(Rotation(Matrix3::identity()))
        .with(Scale(Vector3::new(0.1, 0.1, 0.1)))
        .with(Model(sphere_model))
        .with(LookingAtMarker { player })
        .build();
}
