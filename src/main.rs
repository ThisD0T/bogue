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
    GameState::{Playing, GameEnd},
    GameState,
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

mod ui;
use ui::UiPlugin;

mod debug;
use debug::DebugPlugin;

const WHEIGHT: f32 = 900.0;
const WWIDTH: f32 = 776.0;

//const BG_COLOUR: Color = Color::rgb_u8(219, 130, 96);

fn main() {
    App::new()
    .add_state(GameState::Playing)
    .insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
    .insert_resource(WindowDescriptor {
        title: "Rasteroids".to_string(),
        width: WWIDTH,
        height: WHEIGHT,
        resizable: false,
        ..Default::default()
    })
    .add_system_set(SystemSet::on_update(GameState::Playing)
        .with_system(apply_physics)
    )
    .add_plugins(DefaultPlugins)
    .add_plugin(DebugPlugin)
    .add_plugin(GameObjectPlugin)
    .add_plugin(KillboxPlugin)
    .add_plugin(InputPlugin)
    .add_plugin(UiPlugin)
    .add_startup_system(camera_setup)
    .run();
}

fn camera_setup(
    mut commands: Commands,
) {

    let camera = commands.spawn().id();
    commands.entity(camera)
        .insert_bundle(Camera2dBundle::default());
}
