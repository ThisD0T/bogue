use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    sprite::collide_aabb::collide,
};

use rand;
use rand::{
    Rng,
    thread_rng,
};

mod lib;
use lib::{
    apply_physics,
    SizeVars,
};

mod input;
use input::{
    InputPlugin,
};

mod gameobject;
use gameobject::GameObjectPlugin;
use gameobject::{
    PlayerFlag,
};

mod platform;
use platform::{
    PlatformPlugin
};

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
    .add_plugin(PlatformPlugin)
    .add_plugin(InputPlugin)
    .add_startup_system(camera_setup)
    .add_system(apply_physics)
    .add_system(test_collision)
    .run();
}

fn camera_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    let camera = commands.spawn().id();
    commands.entity(camera)
        .insert_bundle(Camera2dBundle::default());
}

pub fn test_collision(
    mut commands: Commands,
    mut player_query: Query<(&mut SizeVars, &Transform), With<PlayerFlag>>,
    mut platform_query: Query<(&mut SizeVars, &Transform), Without<PlayerFlag>>,
) {
    let (player_size, player_transform) = player_query.single_mut();
    let (platform_size, platform_transform) = platform_query.single_mut();

    let collision = collide(player_transform.translation, player_size.size,
                platform_transform.translation, platform_size.size);

    if collision.is_some() {
        println!("COLLISION");
    }

}