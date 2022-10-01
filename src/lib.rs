use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Playing,
    GameEnd,
}

#[derive(Component)]
pub struct KillboxFlag;

#[derive(Component)]
pub struct PlayerFlag;

#[derive(Component)]
pub struct GameEndTextFlag;

pub const MAX_SPEED: f32 = 3.0;
pub const PLAYER_SPEED: f32 = 0.5;

#[derive(Component)]
pub struct SizeVars{
    pub size: Vec2,
}

#[derive(Component)]
pub struct KillboxTimer{
    pub timer: Timer,
}

#[derive(Component)]
pub struct SpeedTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct KillboxSpeed{
    pub speed: f32,
}

#[derive(Component, Default)]
pub struct PhysVars {
    pub velocity: Vec3,
    pub acceleration: Vec3,
}

#[derive(Component, Default)]
pub struct Health {
    pub health: i32,
}

pub fn apply_physics (
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut PhysVars), With<PhysVars>>,
) {
    for (mut transform, mut physics_vars) in query.iter_mut() {
        let mut temp_acceleration = physics_vars.acceleration;
        let mut temp_vel = physics_vars.velocity;
        let mut decel_vector = temp_vel * -0.1;

        physics_vars.velocity += temp_acceleration;
        physics_vars.velocity += decel_vector;

        physics_vars.velocity = Vec3::clamp(physics_vars.velocity, Vec3::splat(-MAX_SPEED), Vec3::new(MAX_SPEED, MAX_SPEED, 0.0));
        transform.translation += physics_vars.velocity;

        physics_vars.acceleration = Vec3::splat(0.0);
    }
}

pub fn make_text(
    mut commands: Commands,
    assets: Res<AssetServer>,
    string: &str,
    font_size: f32,
    color: Color,
    ui_rect: UiRect<Val>,
) -> Entity {
    let text = commands.spawn().id();
    commands.entity(text)
        .insert_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    
                    string,
                    TextStyle {
                        font: assets.load("LemonMilk.ttf"),
                        font_size: font_size,
                        color: color,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: assets.load("LemonMilk.ttf"),
                    font_size: font_size,
                    color: color,
                }),
            ])
            .with_style(Style{
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: ui_rect,
                ..default()
            }),
        );
    
        return text;
}
