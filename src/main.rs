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
    PlayerFlag,
};

mod input;
use input::{
    InputPlugin,
};

mod gameobject;
use gameobject::GameObjectPlugin;

mod killbox;
use killbox::{
    KillboxPlugin,
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
    .add_plugin(KillboxPlugin)
    .add_plugin(InputPlugin)
    .add_startup_system(camera_setup)
    .add_system(apply_physics)
    .add_system(test_collision)
    .run();
}

fn camera_setup(
    mut commands: Commands,
) {

    let camera = commands.spawn().id();
    commands.entity(camera)
        .insert_bundle(Camera2dBundle::default());
}

pub fn test_collision(
    mut player_query: Query<(&mut SizeVars, &Transform), With<PlayerFlag>>,
    mut killbox_query: Query<(&mut SizeVars, &Transform), Without<PlayerFlag>>,
) {
    let (player_size, player_transform) = player_query.single_mut();
    for (killbox_size, killbox_transform) in killbox_query.iter_mut() {

        let collision = collide(player_transform.translation, player_size.size,
                    killbox_transform.translation, killbox_size.size);

        if collision.is_some() {
        }
    }




}