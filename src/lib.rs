use bevy::prelude::*;

pub const MAX_SPEED: f32 = 3.0;
pub const PLAYER_SPEED: f32 = 0.5;

#[derive(Component)]
pub struct SizeVars{
    pub size: Vec2,
}

#[derive(Component)]
pub struct PlatformTimer{
    timer: Timer,
}

#[derive(Component, Default)]
pub struct PhysVars {
    pub velocity: Vec3,
    pub acceleration: Vec3,
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

