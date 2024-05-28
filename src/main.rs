use bevy::{prelude::*, render::texture, transform::commands};
use bevy_asset_loader::prelude::*;

use debug::*;
use components::*;
use asset_collections::*;

mod debug;
mod components;
mod asset_collections;


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .init_state::<AssetsState>()
    .add_loading_state(
        LoadingState::new(AssetsState::Loading)
        .continue_to_state(AssetsState::Ready)
        .load_collection::<DummyIdleSpriteSheet>()
    )
    .add_systems(OnEnter(AssetsState::Ready), (spawn_camera ,spawn_player))
    .add_plugins(DebugPlugin)
    .add_systems(
        Update,
        animate_sprite_system.run_if(in_state(AssetsState::Ready)),
    )
    .run();
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());  
}

pub fn spawn_player(
    mut commands: Commands,
    assets: Res<DummyIdleSpriteSheet>,
) {
    commands.spawn((
        SpriteBundle{
            texture: assets.dummy.clone(),
            transform: Transform::from_xyz(0., 150., 0.),
            ..Default::default()
        },
        TextureAtlas::from(assets.dummy_layout.clone()),
        AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
    ));
}
fn animate_sprite_system(
    time: Res<Time>,
    mut sprites_to_animate: Query<(&mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (mut timer, mut sprite) in &mut sprites_to_animate {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index = (sprite.index + 1) % 4;
        }
    }
}