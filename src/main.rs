use bevy::{
    prelude::*,
    render::camera::ScalingMode
};

use rand;
use rand::{
    Rng,
    thread_rng,
};

mod lib;
use lib::{
    apply_physics,
};

mod input;
use input::{
    InputPlugin,
};

mod gameobject;
use gameobject::GameObjectPlugin;

mod debug;
use debug::DebugPlugin;

const WHEIGHT: f32 = 600.0;
const WWIDTH: f32 = 576.0;

//const BG_COLOUR: Color = Color::rgb_u8(219, 130, 96);

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
    .insert_resource(WindowDescriptor {
        title: "Rasteroids".to_string(),
        width: WWIDTH,
        height: WHEIGHT,
        resizable: false,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(DebugPlugin)
    .add_plugin(GameObjectPlugin)
    .add_plugin(InputPlugin)
    .add_startup_system(camera_setup)
    .add_system(apply_physics)
    .run();
}

fn camera_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    let camera = commands.spawn().id();
    commands.entity(camera)
        .insert_bundle(Camera2dBundle::default());
    
    let background = commands.spawn().id();
    commands.entity(background)
        .insert_bundle(SpriteBundle{
            transform: Transform{
                translation: Vec3::splat(0.0),
                ..default()
            },
            texture: assets.load("background.png"),
            ..default()
        });
}
