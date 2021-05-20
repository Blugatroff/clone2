#![feature(stmt_expr_attributes)]
#![warn(rust_2018_idioms)]
#![forbid(unsafe_code)]
mod blocks;
mod chunk;
mod chunk_middle_ware;
mod components;
mod dir;
mod flat_middleware;
mod manager;
mod renderer;
mod resources;
mod setup;
mod systems;

use blocks::Atlas;
use cgmath::{InnerSpace, Matrix3, Rad, Vector2, Vector3};
use components::{RealLight, Scale};
use finger_paint_wgpu::Camera;
use finger_paint_wgpu::{
    text::{HorizontalAlign, Paragraph, TextSection, VerticalAlign},
    RealLightApi, Resize, SimpleLightApi,
};
use renderer::Renderer;
use resources::DeltaTime;
use simple_winit::input::{Input, VirtualKeyCode};
use simple_winit::winit::window::Window;
use specs::prelude::ComponentEvent;
use specs::{DispatcherBuilder, Entity, Join, ReaderId, RunNow, SystemData, World, WorldExt};
use std::time::Duration;

pub struct State {
    world: World,
    time: f32,
    vsync: bool,
    atlas: Atlas,
    real_light_component_channel: ReaderId<ComponentEvent>,
    cross_hair: Entity,
    renderer: Renderer,
}

use crate::{chunk::Chunk, manager::ModelManager};
use setup::*;

impl State {
    fn new(window: &Window) -> Self {
        let mut renderer = Renderer::new(&window);

        let mut world = World::new();
        setup::setup(&mut world);
        let real_light_component_channel =
            specs::WriteStorage::<RealLight>::fetch(&world).register_reader();
        renderer.text_middleware.paragraphs().push(Paragraph {
            vertical_alignment: VerticalAlign::Top,
            horizontal_alignment: HorizontalAlign::Left,
            position: Vector2::new(0.0, 0.0),
            sections: vec![TextSection {
                text: "".into(),
                color: [1.0, 1.0, 1.0, 1.0],
                scale: 15.0,
                font: Default::default(),
            }],
        });

        setup_highlight_cube(&mut world, &mut renderer);
        let sphere = renderer
            .model_middleware
            .load_model_obj("./res/sphere.obj")
            .unwrap();
        let sphere = world.fetch_mut::<ModelManager>().insert(sphere);
        setup_player(&mut world, sphere);
        let cross_hair = setup_cross_hair(&mut world, &mut renderer);

        let (_, queue) = renderer.renderer.device_and_queue();
        renderer
            .chunk_mesh_middleware
            .load_atlas(queue, "atlas.png");
        let atlas = Atlas::load();
        renderer.chunk_mesh_middleware.load_uvs(&atlas.all_uvs);
        let vsync = true;
        renderer.renderer.enable_vsync(vsync);
        renderer.renderer.set_shadow_resolution([1024, 1024]);
        renderer
            .renderer
            .set_ambient_light(Vector3::new(1.0, 1.0, 1.0));

        Self {
            renderer,
            real_light_component_channel,
            world,
            time: 0.0,
            vsync,
            atlas,
            cross_hair,
        }
    }
}

impl simple_winit::WindowLoop for State {
    fn update(&mut self, input: &mut Input, dt: Duration) {
        input.grab_cursor(true);
        let dt = dt.as_secs_f32();
        self.time += dt;
        *self.world.write_resource::<DeltaTime>() = DeltaTime(dt);
        if let Some(size) = input.resized() {
            self.renderer
                .text_middleware
                .resize((size.0 as u32, size.1 as u32));
            self.renderer.renderer.resize(size);
            self.world
                .write_component::<Scale>()
                .get_mut(self.cross_hair)
                .unwrap()
                .0 = Vector3::new(1.0, 1.0 * self.renderer.renderer.aspect(), 1.0) * 0.0125;
        }

        self.renderer.text_middleware.paragraphs()[0].sections[0].text = format!(
            "direction: {:?}\nposition:{:?}\nfps:{}\n",
            self.world.fetch::<Camera>().get_direction(),
            self.world.fetch::<Camera>().get_position(),
            1.0 / dt
        );

        if input.key_pressed(VirtualKeyCode::V) {
            self.vsync = !self.vsync;
            self.renderer.renderer.enable_vsync(self.vsync);
        }
        if input.key_pressed(VirtualKeyCode::Y) {
            let mut chunks = self.world.write_storage::<Chunk>();
            let entities = self.world.entities();
            let mut to_be_removed = Vec::new();
            for (_, entity) in (&chunks, &entities).join() {
                to_be_removed.push(entity);
            }
            for entity in to_be_removed {
                chunks.remove(entity);
            }
        }

        #[rustfmt::skip]
        {
            DispatcherBuilder::new()
                .with(systems::VelocitySystem, "VelocitySystem", &[])
                .with(systems::FirstPersonController(input), "FirstPersonController", &["VelocitySystem"])
                .with(systems::ThirdPersonCameraSystem, "ThirdPersonCameraSystem", &["FirstPersonController"])
                .with(systems::UpdateCameras, "UpdateCameras", &["FirstPersonController"])
                .with(systems::TransformRealLights, "TransformRealLights", &[])
                .with(systems::RemoveChunks, "RemoveChunks", &["FirstPersonController"])
                .build()
                .dispatch(&self.world);
        }
        systems::GenerateChunks(&mut self.renderer).run_now(&self.world);
        systems::BreakBlocks(input).run_now(&self.world);
        systems::UpdateNeighbouringChunks.run_now(&self.world);
        self.world.maintain();
        systems::LookingAtSystem(&mut self.renderer.lines).run_now(&self.world);
        systems::LookingAtMarkerSystem.run_now(&self.world);
        systems::UpdateChunks(&mut self.renderer).run_now(&self.world);
        systems::RenderUvMeshes.run_now(&self.world);
        systems::RenderModels(
            &mut self.renderer.renderer,
            &mut self.renderer.model_middleware,
        )
        .run_now(&self.world);
        systems::RenderFlatMeshes(&mut self.renderer.flat_middleware).run_now(&self.world);
        systems::ChunkMeshGeneration(&mut self.renderer.chunk_mesh_middleware, &mut self.atlas)
            .run_now(&self.world);
        systems::UpdateRealLights::new(&mut self.renderer, &mut self.real_light_component_channel)
            .run_now(&self.world);
        systems::UpdateCamera(&mut self.renderer).run_now(&self.world);

        self.world.maintain();
    }
    fn render(&mut self, window: &Window) {
        let a = self.renderer.renderer.aspect();
        self.renderer.renderer.camera().set_aspect_ratio(a);
        self.renderer.renderer.update();
        self.renderer.render(window, &self.world);
    }
}

pub fn main() {
    let (window, event_loop) = simple_winit::create("clone_v2");

    simple_winit::start(State::new(&window), (window, event_loop));
}

fn rotation_matrix_from_direction(d: Vector3<f32>) -> Matrix3<f32> {
    let d = d.normalize();
    Matrix3::from_angle_y(Rad(-d.z.atan2(d.x)))
        * Matrix3::from_angle_z(Rad(d.y.atan2((d.x * d.x + d.z * d.z).sqrt())))
}
