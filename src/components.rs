use bevy::prelude::*;

#[derive(Component)]
pub struct Background;

// components.rs
#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct GetReady;

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct Tap {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Score;

#[derive(Component)]
pub struct Bird{
    pub timer: Timer,
}

#[derive(Component)]
pub struct TopPipe;

#[derive(Component)]
pub struct BottomPipe;