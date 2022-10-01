use bevy::prelude::*;

use crate::lib::{
    make_text,
    GameState::{Playing, GameEnd},
    GameState,
    GameEndTextFlag,
    PlayerFlag,
    Health,
};

pub struct UiPlugin;

impl Plugin for UiPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(init_text)
            .add_system_set(SystemSet::on_update(GameState::GameEnd)
                .with_system(game_end)
            )
            .add_system(update_text);
    }
}

fn init_text(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let font_size = 30.0;

    let health_text = commands.spawn().id();
    commands.entity(health_text)
        .insert_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    
                    "Health: ",
                    TextStyle {
                        font: assets.load("LemonMilk.ttf"),
                        font_size: font_size,
                        color: Color::GREEN,
                    },
                ),
                TextSection::from_style(TextStyle {
                    font: assets.load("LemonMilk.ttf"),
                    font_size: font_size,
                    color: Color::GREEN,
                }),
            ])
            .with_style(Style{
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(9.0),
                    left: Val::Px(9.0),
                    ..default()
                },
                ..default()
            }),
        ).insert(GameEndTextFlag);
}

fn game_end(
    mut commands: Commands,
    assets: Res<AssetServer>
) {

}

fn update_text(
    mut commands: Commands,
    mut query: Query<&mut Text, With<GameEndTextFlag>>,
    mut player_query: Query<&mut Health, With<PlayerFlag>>,
    state: ResMut<State<GameState>>,
) {
    let mut text = query.single_mut();
    let mut health = player_query.single_mut();

    text.sections[1].value = format!("{}", health.health);
}
