mod prefs;
mod components;
mod utils;
mod sys;
mod setup;

use bevy::prelude::*;
use crate::{
    setup::setup,
    prefs::{WINDOW_HEIGHT, WINDOW_WIDTH},
    sys::*
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.188, 0.753, 0.875)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title:"Flappy Bird".to_string(),
                resolution: (WINDOW_WIDTH,WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, animate_tap)
        .add_systems(Update, move_background)
        .add_systems(Update, move_ground)
        .add_systems(Update, animate_bird)
        .run();
}
