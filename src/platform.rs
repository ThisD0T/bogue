use bevy::prelude::*;

use rand::thread_rng;

use crate::lib::{
    SizeVars,
    PlatformTimer,
};

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(test_setup);
    }
}

fn test_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    spawn_platform(commands, assets);
}

fn platform_generator(
    mut commands: Commands,
    assets: Res<AssetServer>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    let mut platform_timer = query.single_mut();

    platform_timer.timer.tick(time.delta_seconds());
    
}

fn spawn_platform(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    let platform = commands.spawn().id();

    commands.entity(platform)
        .insert_bundle(SpriteBundle{
            sprite: Sprite{
                custom_size: Some(Vec2::new(50.0, 90.0)),
                ..default()
            },
            transform: Transform{
                translation: Vec3::splat(0.0),
                ..default()
            },
            texture: assets.load("platform.png"),
            ..default()
        })
        .insert(SizeVars{size: Vec2::new(50.0, 90.0)})
        .insert(Name::new("Platform"));
}
