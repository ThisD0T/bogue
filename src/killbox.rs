use bevy::{
    prelude::*,
    sprite::collide_aabb::collide,
};

use rand::{
    thread_rng,
    Rng,
};

use crate::{
    WHEIGHT,
    WWIDTH,
};

use crate::lib::{
    SizeVars,
    KillboxTimer,
    KillboxFlag,
    PlayerFlag,
    SpeedTimer,
    KillboxSpeed,
    Health,
    GameState::{Playing, GameEnd},
    GameState,
};

pub struct KillboxPlugin;

impl Plugin for KillboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(test_setup);
        app
            .add_system_set(SystemSet::on_update(GameState::Playing)
                .with_system(killbox_generator)
                .with_system(move_killboxes)
            )
            .add_system_set(SystemSet::on_update(GameState::GameEnd)
            )
            .add_system(despawn_killboxes)
            .add_system(killbox_collision);
    }
}

fn test_setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    spawn_killbox(commands, assets, Vec3::splat(0.0), Vec2::new(100.0, 50.0));
}


fn spawn_killbox(
    mut commands: Commands,
    assets: Res<AssetServer>,
    position: Vec3,
    size: Vec2,
) {

    let killbox = commands.spawn().id();

    commands.entity(killbox)
        .insert_bundle(SpriteBundle{
            sprite: Sprite{
                custom_size: Some(Vec2::new(50.0, 90.0)),
                ..default()
            },
            transform: Transform{
                translation: Vec3::splat(0.0),
                ..default()
            },
            texture: assets.load("killbox.png"),
            ..default()
        })
        .insert(SizeVars{size: Vec2::new(50.0, 90.0)})
        .insert(KillboxFlag)
        .insert(Name::new("killbox"));
}

fn move_killboxes(
    mut query: Query<&mut Transform, With<KillboxFlag>>,
    mut timer_query: Query<(&mut SpeedTimer, &mut KillboxSpeed), With<PlayerFlag>>,
    time: Res<Time>,
) {
    let (mut timer, mut speed) = timer_query.single_mut();

    timer.timer.tick(time.delta());

    if timer.timer.just_finished() {
        speed.speed += 0.2;
        timer.timer.reset();
    }
    
    for mut transform in query.iter_mut() {
        transform.translation.y -= speed.speed;
    }
}

fn killbox_generator(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut SizeVars), With<KillboxFlag>>,
    mut player_query: Query<&mut KillboxTimer, With<PlayerFlag>>,
    assets: Res<AssetServer>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    let mut timer = player_query.single_mut();

    timer.timer.tick(time.delta());

    let killbox_spacing: f32 = 60.0;

    let mut killboxes: i32 = 0;
    for (_entity, _transform, _size_vars) in query.iter() {
        killboxes += 1;
    }

    if killboxes < 29 {

        let rand_size: Vec2 = Vec2::new(rng.gen_range(20.0..80.0), rng.gen_range(20.0..80.0));
        let position = Vec3::new(rng.gen_range(-WWIDTH..WWIDTH)/2.0, rng.gen_range((WHEIGHT/2.0)..(WHEIGHT*2.0)), 0.0);

        let mut computed = false;

        while computed == false{

            if query.is_empty() {
                break;
            }

            for (_entity, other_transform, other_size) in query.iter() {
                computed = true;
                let position = Vec3::new(rng.gen_range(-WWIDTH/2.0..WWIDTH/2.0), rng.gen_range((WHEIGHT/2.0)..WHEIGHT), 0.0);

                let top_point = other_transform.translation.y + other_size.size.y/2.0;
                let _bottom_point: f32 = other_transform.translation.y - other_size.size.x/2.0;
                let left_point = other_transform.translation.x - other_size.size.x/2.0;
                let right_point = other_transform.translation.x + other_size.size.x/2.0;

                let _top = position.y + rand_size.y/2.0;
                let bottom: f32 = position.y - rand_size.x/2.0;
                let left = position.x - rand_size.x/2.0;
                let right = position.x + rand_size.x/2.0;

                if bottom > top_point + (killbox_spacing * 2.0) || left > right_point + killbox_spacing || right < left_point - killbox_spacing {
                    computed = true;
                }
            }
        }
        let killbox = commands.spawn().id();

        commands.entity(killbox)
            .insert_bundle(SpriteBundle{
                sprite: Sprite{
                    custom_size: Some(rand_size),
                    ..default()
                },
                transform: Transform{
                    translation: position,
                    ..default()
                },
                texture: assets.load("killbox.png"),
                ..default()
            })
            .insert(SizeVars{size: rand_size})
            .insert(KillboxFlag)
            .insert(Name::new("killbox"));
        
        timer.timer.reset();
    }
}

fn despawn_killboxes(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<KillboxFlag>>,
) {

    for (entity, transform) in query.iter_mut() {
        if transform.translation.y < -WHEIGHT {
            commands.entity(entity).despawn();
        }
    }
}

pub fn killbox_collision(
    mut commands: Commands,
    mut player_query: Query<(&mut SizeVars, &Transform, &mut Health), With<PlayerFlag>>,
    mut killbox_query: Query<(Entity, &mut SizeVars, &Transform), Without<PlayerFlag>>,
    mut state: ResMut<State<GameState>>,
) {
    let (player_size, player_transform, mut player_health) = player_query.single_mut();

    for (killbox, killbox_size, killbox_transform) in killbox_query.iter_mut() {

        let collision = collide(player_transform.translation, player_size.size,
                    killbox_transform.translation, killbox_size.size);

        if collision.is_some() {
            commands.entity(killbox).despawn();
            player_health.health -= 1;
            if player_health.health < 1 {
                state.set(GameState::GameEnd).expect("failed to change game state");
            }
            println!("{}", player_health.health);
        }
    }




}