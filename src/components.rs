use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssetsState{
    #[default]
    Loading,
    Ready,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PausedState{
    #[default]
    Paused,
    Running,
}

#[derive(Component)]
pub struct AnimationTimer(
    pub Timer
);

#[derive(Component)]
pub struct Player{
    hp: f32,
    speed: f32,
    damage: f32,
    firerate: f32,
    luck: f32,
}