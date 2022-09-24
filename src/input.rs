use bevy::{
    prelude::*,
    input::{
        ButtonState,
        keyboard::KeyboardInput,
    },
};

use crate::lib::{
    PhysVars,
    PLAYER_SPEED,
};

use crate::gameobject::{
    PlayerFlag,
};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_input);
    }
}

struct MoveDirection {
    left: Vec3,
    right: Vec3,
    up: Vec3,
    down: Vec3,
}

fn player_input(
    mut player_query: Query<(&mut Transform, &mut PhysVars), With<PlayerFlag>>,
    keys: Res<Input<KeyCode>>,
) {
    let (mut transform, mut phys_vars) = player_query.single_mut();

    let move_direction = MoveDirection{
        left: Vec3::new(-PLAYER_SPEED, 0.0, 0.0),
        right: Vec3::new(PLAYER_SPEED, 0.0, 0.0),
        up: Vec3::new(0.0, PLAYER_SPEED, 0.0),
        down: Vec3::new(0.0, -PLAYER_SPEED, 0.0),
    };

    if keys.pressed(KeyCode::A) {
        phys_vars.acceleration += move_direction.left;
    } if keys.pressed(KeyCode::D) {
        phys_vars.acceleration += move_direction.right;
    }

    if keys.pressed(KeyCode::W) {
        phys_vars.acceleration += move_direction.up;
    } if keys.pressed(KeyCode::S) {
        phys_vars.acceleration += move_direction.down;
    }

}
