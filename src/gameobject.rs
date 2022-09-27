use bevy::prelude::*;
use bevy::{
    sprite::collide_aabb::collide,

};

use crate::lib::{
    PhysVars,
    SizeVars,
    PlatformTimer,
};

const PLAYER_SIZE: f32 = 25.0;

#[derive(Component)]
pub struct PlayerFlag;


pub struct GameObjectPlugin;

impl Plugin for GameObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(make_player);
    }
}

fn make_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {

    let player = commands.spawn().id();
    commands.entity(player)
        .insert_bundle(SpriteBundle{
            sprite: Sprite{
                ..default()
            },
            texture: assets.load("player.png"),
            transform: Transform{
                translation: Vec3::new(150.0, 0.0, 100.0),
                ..default()
            },
            ..default()
        })
        .insert(SizeVars{size: Vec2::new(20.0, 20.0)})
        .insert(PhysVars::default())
        .insert(PlayerFlag)
        .insert(Name::new("Player"));
}
