use bevy::prelude::*;

use bevy_egui::{egui, EguiContext, EguiPlugin};

use bevy_inspector_egui::{
    Inspectable, RegisterInspectable, WorldInspectorParams, WorldInspectorPlugin,
};

const WINDOW_HEIGHT: f32 = 720.;
const WINDOW_RESOLUTION: f32 = 16./9.;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: WINDOW_HEIGHT * WINDOW_RESOLUTION,
            height: WINDOW_HEIGHT,
            title: "jam".to_string(),
            resizable: false,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .run();
}

//this spawns a camera with a bottom to top Y coord system and a left to right X coord system
fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;

    camera.orthographic_projection.left = 0.;
    camera.orthographic_projection.right = WINDOW_HEIGHT * WINDOW_RESOLUTION;

    camera.orthographic_projection.top = WINDOW_HEIGHT;
    camera.orthographic_projection.bottom = 0.;

    commands.spawn_bundle(camera);
}
