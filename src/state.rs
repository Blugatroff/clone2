use crate::resources::DeltaTime;
use crate::systems;
use crate::{
    blocks::Atlas,
    components::{Position, Rotation},
};
use crate::{components::Model, manager::UvMeshManager};
use crate::{
    components::{ChunkMesh, FlatMesh, RealLight, Scale, Sun},
    resources::Time,
};
use cgmath::{InnerSpace, SquareMatrix};
use cgmath::{Matrix3, Vector2, Vector3};
use finger_paint_wgpu::{
    lines::Lines, model::ModelMiddleWare, text::TextMiddleWare, uv_mesh::UvMeshMiddleWare, Camera,
    LightAttenuation, Resize, WgpuRenderer,
};
use finger_paint_wgpu::{
    text::{HorizontalAlign, Paragraph, TextSection, VerticalAlign},
    RealLightApi, SimpleLightApi,
};
use simple_winit::input::{Input, VirtualKeyCode};
use simple_winit::winit::window::Window;
use specs::{Builder, DispatcherBuilder, Entity, Join, World, WorldExt};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

pub const DAY_LENGTH: f32 = 60.0;
#[allow(dead_code)]
pub struct State {
    world: World,
    time: f32,
    vsync: bool,
    cross_hair: Entity,

    text_middleware: TextMiddleWare,
    lines: Lines,
    model_middleware: ModelMiddleWare,
}

use crate::setup::*;
use crate::{
    chunk::Chunk, chunk_middle_ware::ChunkMeshMiddleWare, flat_middleware::FlatMiddleWare,
    manager::ModelManager,
};

impl State {
    pub fn new(window: &Window) -> Self {
        let mut renderer = WgpuRenderer::new(window, false);
        renderer.set_shadow_resolution([2048, 2048]);
        renderer.clear_color([0.2, 0.4, 1.0, 1.0]);
        let chunk_mesh_middleware = renderer.load_middle_ware::<ChunkMeshMiddleWare>();
        let mut text_middleware = renderer.load_middle_ware::<TextMiddleWare>();
        let flat_middleware = renderer.load_middle_ware::<FlatMiddleWare>();
        let uv_mesh_middleware = renderer.load_middle_ware::<UvMeshMiddleWare>();
        let lines = renderer.load_middle_ware::<Lines>();
        let model_middleware = renderer.load_middle_ware::<ModelMiddleWare>();

        let mut world = World::new();
        setup(&mut world);
        world.insert(renderer);
        world.insert(chunk_mesh_middleware);
        world.insert(flat_middleware);
        world.insert(uv_mesh_middleware);

        text_middleware.paragraphs().push(Paragraph {
            vertical_alignment: VerticalAlign::Top,
            horizontal_alignment: HorizontalAlign::Left,
            position: Vector2::new(0.0, 0.0),
            sections: vec![TextSection {
                text: "".into(),
                color: [0.0, 0.0, 0.0, 1.0],
                scale: 20.0,
                font: Default::default(),
            }],
        });

        let mut model = model_middleware
            .load_model("../libs/finger_paint_wgpu/res/sponza.glb")
            .unwrap();
        model.lighting(true);
        let model = world.fetch_mut::<ModelManager>().insert(model);
        world
            .create_entity()
            .with(Model(model))
            .with(Position(Vector3::new(-10.0, 20.0, 20.0)))
            .with(Rotation(Matrix3::identity()))
            .build();

        let sphere = model_middleware.load_model_obj("./res/sphere.obj").unwrap();
        let sphere = world.fetch_mut::<ModelManager>().insert(sphere);
        let player = setup_player(&mut world, sphere);
        let cross_hair = setup_cross_hair(&mut world);

        let mut renderer = world.fetch_mut::<WgpuRenderer>();
        let queue = renderer.queue();

        let sun_light = renderer.add_real_light(finger_paint_wgpu::RealLight {
            camera: finger_paint_wgpu::Camera::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(10.0, 30.0, 10.0).normalize(),
                Vector3::unit_y(),
                finger_paint_wgpu::ViewMatrixMode::Orthographic {
                    near: 0.1,
                    far: 128.0,
                    left: -32.0,
                    right: 32.0,
                    bottom: -32.0,
                    top: 32.0,
                },
            ),
            color: [1.0, 1.0, 1.0, 1.0],
            default: 0.0,
            attenuation: LightAttenuation::default(),
            enabled: true,
        });
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
        renderer.set_ambient_light(Vector3::new(1.0, 1.0, 1.0) * 0.2);
        renderer.enable_shadows(true);
        drop(renderer);
        world.insert(atlas);
        world
            .create_entity()
            .with(Position(Vector3::new(-10.0, 20.0, 20.0)))
            .with(Rotation(Matrix3::identity()))
            .with(RealLight(sun_light))
            .with(Sun {
                player,
                distance: 64.0,
                size: 64.0,
            })
            .with(Model(sphere))
            .build();
        Self {
            world,
            time: 0.0,
            vsync,
            cross_hair,

            text_middleware,
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
        self.world.fetch_mut::<Time>().update();
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
            .with(systems::VelocitySystem          , "VelocitySystem"          , &[                          ])
            .with(systems::FirstPersonController   , "FirstPersonController"   , &["VelocitySystem"          ])
            .with(systems::ThirdPersonCameraSystem , "ThirdPersonCameraSystem" , &["FirstPersonController"   ])
            .with(systems::UpdateCameras           , "UpdateCameras"           , &["ThirdPersonCameraSystem" ])
            .with(systems::TransformRealLights     , "TransformRealLights"     , &[                          ])
            .with(systems::GenerateChunks          , "GenerateChunks"          , &[                          ])
            .with(systems::BreakBlocks             , "BreakBlocks"             , &["FirstPersonController"   ])
            .with(systems::PlaceBlocks             , "PlaceBlocks"             , &["BreakBlocks"             ])
            .with(systems::UpdateNeighbouringChunks, "UpdateNeighbouringChunks", &["PlaceBlocks"             ])
            .with(systems::RemoveChunks            , "RemoveChunks"            , &["UpdateNeighbouringChunks"])
            .with(systems::LookingAtSystem         , "LookingAtSystem"         , &["RemoveChunks"            ])
            .with(systems::LookingAtMarkerSystem   , "LookingAtMarkerSystem"   , &["LookingAtSystem"         ])
            .with(systems::BlockHighlighting       , "BlockHighlighting"       , &["LookingAtSystem"         ])
            .with(systems::RenderUvMeshes          , "RenderUvMeshes"          , &["BlockHighlighting"       ])
            .with(systems::RenderModels            , "RenderModels"            , &[                          ])
            .with(systems::RenderFlatMeshes        , "RenderFlatMeshes"        , &[                          ])
            .with(systems::ChunkMeshGeneration     , "ChunkMeshGeneration"     , &[                          ])
            .with(systems::SunSystem               , "SunSystem"               , &[                          ])
            .with(systems::UpdateCamera            , "UpdateCamera"            , &["ChunkMeshGeneration"     ])
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
        renderer.update_real_lights();

        let model_manager = self.world.fetch_mut::<ModelManager>();
        let models = model_manager.get_all();
        let uv_mesh_manager = self.world.fetch_mut::<UvMeshManager>();
        let uv_meshes = uv_mesh_manager.get_all();
        let chunks = self.world.read_component::<ChunkMesh>();
        let chunks = chunks.as_slice().iter().map(|m| &m.0);
        let flat_middleware = self.world.fetch::<FlatMiddleWare>();
        let flat_meshes = self.world.read_component::<FlatMesh>();
        let flat_meshes = flat_meshes.as_slice().iter().map(|m| &m.0);
        let uv_mesh_middleware = self.world.fetch::<UvMeshMiddleWare>();

        renderer.render(
            window,
            &mut [
                &mut self.world.fetch::<ChunkMeshMiddleWare>().prepare(chunks),
                &mut self.model_middleware.prepare(models),
                &mut uv_mesh_middleware.prepare(uv_meshes),
                &mut self.text_middleware,
                &mut flat_middleware.prepare(flat_meshes),
                &mut self.lines,
            ],
        );
    }
}
