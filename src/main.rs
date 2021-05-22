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
mod resources;
mod setup;
mod systems;

use blocks::Atlas;
use cgmath::{InnerSpace, Matrix3, Rad, Vector2, Vector3};
use components::{ChunkMesh, FlatMesh, RealLight, Scale};
use finger_paint_wgpu::{
    lines::Lines, model::ModelMiddleWare, text::TextMiddleWare, uv_mesh::UvMeshMiddleWare, Camera,
    Resize, WgpuRenderer,
};
use finger_paint_wgpu::{
    text::{HorizontalAlign, Paragraph, TextSection, VerticalAlign},
    RealLightApi, SimpleLightApi,
};
use manager::UvMeshManager;
use resources::DeltaTime;
use simple_winit::input::{Input, VirtualKeyCode};
use simple_winit::winit::window::Window;
use specs::{DispatcherBuilder, Entity, Join, SystemData, World, WorldExt};
use ton::Player;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

#[allow(dead_code)]
pub struct State {
    world: World,
    time: f32,
    vsync: bool,
    cross_hair: Entity,

    text_middleware: TextMiddleWare,
    uv_mesh_middleware: UvMeshMiddleWare,
    lines: Lines,
    model_middleware: ModelMiddleWare,
}

use crate::{
    chunk::Chunk, chunk_middle_ware::ChunkMeshMiddleWare, flat_middleware::FlatMiddleWare,
    manager::ModelManager,
};
use setup::*;

impl State {
    fn new(window: &Window) -> Self {
        let player = Player::new();

        let renderer = WgpuRenderer::new(window, false);
        let chunk_mesh_middleware = renderer.load_middle_ware::<ChunkMeshMiddleWare>();
        let mut text_middleware = renderer.load_middle_ware::<TextMiddleWare>();
        let flat_middleware = renderer.load_middle_ware::<FlatMiddleWare>();
        let mut uv_mesh_middleware = renderer.load_middle_ware::<UvMeshMiddleWare>();
        let lines = renderer.load_middle_ware::<Lines>();
        let model_middleware = renderer.load_middle_ware::<ModelMiddleWare>();

        let mut world = World::new();
        setup::setup(&mut world);
        world.insert(renderer);
        world.insert(chunk_mesh_middleware);
        world.insert(flat_middleware);

        let real_light_component_channel =
            specs::WriteStorage::<RealLight>::fetch(&world).register_reader();
        world.insert(real_light_component_channel);
        text_middleware.paragraphs().push(Paragraph {
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

        setup_highlight_cube(&mut world, &mut uv_mesh_middleware);
        let sphere = model_middleware.load_model_obj("./res/sphere.obj").unwrap();
        let sphere = world.fetch_mut::<ModelManager>().insert(sphere);
        setup_player(&mut world, sphere);
        let cross_hair = setup_cross_hair(&mut world);

        let mut renderer = world.fetch_mut::<WgpuRenderer>();
        let queue = renderer.queue();

        world
            .fetch_mut::<ChunkMeshMiddleWare>()
            .load_atlas(&queue, "atlas.png");
        let atlas = Atlas::load();
        world
            .fetch_mut::<ChunkMeshMiddleWare>()
            .load_uvs(&atlas.all_uvs);
        let vsync = true;
        renderer.enable_vsync(vsync);
        renderer.set_shadow_resolution([1024, 1024]);
        renderer.set_ambient_light(Vector3::new(1.0, 1.0, 1.0));
        drop(renderer);
        world.insert(atlas);
        world.insert(player);

        Self {
            world,
            time: 0.0,
            vsync,
            cross_hair,

            text_middleware,
            uv_mesh_middleware,
            lines,
            model_middleware,
        }
    }
}

impl simple_winit::WindowLoop for State {
    fn init(&mut self, input: Arc<Mutex<Input>>) {
        self.world.insert(input);
    }
    fn update(&mut self, dt: Duration) {
        let input_arc = self.world.fetch::<Arc<Mutex<Input>>>();
        let mut input = input_arc.lock().unwrap();
        if input.key_pressed(VirtualKeyCode::M) {
            input.grab_cursor(true);
        }
        if input.key_pressed(VirtualKeyCode::J) {
            input.grab_cursor(false);
        }
        let dt = dt.as_secs_f32();
        self.time += dt;
        *self.world.write_resource::<DeltaTime>() = DeltaTime(dt);
        let mut renderer = self.world.fetch_mut::<WgpuRenderer>();
        if let Some(size) = input.resized() {
            self.text_middleware.resize((size.0 as u32, size.1 as u32));
            renderer.resize(size);
            self.world
                .write_component::<Scale>()
                .get_mut(self.cross_hair)
                .unwrap()
                .0 = Vector3::new(1.0, 1.0 * renderer.aspect(), 1.0) * 0.0125;
        }

        self.text_middleware.paragraphs()[0].sections[0].text = format!(
            "direction: {:?}\nposition:{:?}\nfps:{}\n",
            self.world.fetch::<Camera>().get_direction(),
            self.world.fetch::<Camera>().get_position(),
            1.0 / dt
        );

        if input.key_pressed(VirtualKeyCode::V) {
            self.vsync = !self.vsync;
            renderer.enable_vsync(self.vsync);
        }
        if input.key_pressed(VirtualKeyCode::Y) {
            let chunks = self.world.read_storage::<Chunk>();
            let entities = self.world.entities();
            let mut to_be_removed = Vec::new();
            for (_, entity) in (&chunks, &entities).join() {
                to_be_removed.push(entity);
            }
            for entity in to_be_removed {
                entities.delete(entity).unwrap();
            }
        }
        drop(input);
        drop(input_arc);
        drop(renderer);
        #[rustfmt::skip]
        {
        DispatcherBuilder::new()
            .with(systems::VelocitySystem, "VelocitySystem", &[])
            .with(systems::FirstPersonController, "FirstPersonController", &["VelocitySystem"])
            .with(systems::ThirdPersonCameraSystem, "ThirdPersonCameraSystem", &["FirstPersonController"])
            .with(systems::UpdateCameras, "UpdateCameras", &["ThirdPersonCameraSystem"])
            .with(systems::TransformRealLights, "TransformRealLights", &[])
            .with(systems::GenerateChunks, "GenerateChunks", &[])
            .with(systems::BreakBlocks, "BreakBlocks", &["FirstPersonController"])
            .with(systems::PlaceBlocks, "PlaceBlocks", &["BreakBlocks"])
            .with(systems::UpdateNeighbouringChunks, "UpdateNeighbouringChunks", &["PlaceBlocks"])
            .with(systems::RemoveChunks, "RemoveChunks", &["UpdateNeighbouringChunks"])
            .with(systems::LookingAtSystem, "LookingAtSystem", &["RemoveChunks"])
            .with(systems::LookingAtMarkerSystem, "LookingAtMarkerSystem", &["LookingAtSystem"])
            .with(systems::RenderUvMeshes, "RenderUvMeshes", &["LookingAtMarkerSystem"])
            .with(systems::RenderModels, "RenderModels", &["RenderUvMeshes"])
            .with(systems::RenderFlatMeshes, "RenderFlatMeshes", &["RenderModels"])
            .with(systems::ChunkMeshGeneration, "ChunkMeshGeneration", &["RenderFlatMeshes"])
            .with(systems::UpdateCamera, "UpdateCamera", &["ChunkMeshGeneration"])
            .with(systems::UpdateRealLights, "UpdateRealLights", &["UpdateCamera"])
            .build()
            .dispatch(&self.world);
        }
        self.world.maintain();
    }
    fn render(&mut self, window: &Window) {
        let mut renderer = self.world.fetch_mut::<WgpuRenderer>();
        let a = renderer.aspect();
        renderer.camera().set_aspect_ratio(a);
        renderer.update();

        let model_manager = self.world.fetch_mut::<ModelManager>();
        let models = model_manager.get_all();
        let uv_mesh_manager = self.world.fetch_mut::<UvMeshManager>();
        let uv_meshes = uv_mesh_manager.get_all();
        let chunks = self.world.read_component::<ChunkMesh>();
        let chunks = chunks.as_slice().iter().map(|m| &m.0);
        let flat_middleware = self.world.fetch::<FlatMiddleWare>();
        let flat_meshes = self.world.read_component::<FlatMesh>();
        let flat_meshes = flat_meshes.as_slice().iter().map(|m| &m.0);

        renderer.render(
            window,
            &mut [
                &mut self.world.fetch::<ChunkMeshMiddleWare>().prepare(chunks),
                &mut self.model_middleware.prepare(models),
                &mut self.uv_mesh_middleware.prepare(uv_meshes),
                &mut self.text_middleware,
                &mut flat_middleware.prepare(flat_meshes),
                &mut self.lines,
            ],
        );
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
