mod prefs;
mod components;
mod utils;
mod sys;
mod setup;
mod resources;

use bevy::prelude::*;
use crate::{
    setup::setup,
    prefs::{WINDOW_HEIGHT, WINDOW_WIDTH},
    sys::*,
    resources::*
};

fn main() {
    App::new()
        .init_resource::<Game>()
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
        .add_systems(Update, animate_tap.run_if(is_game_not_active))
        .add_systems(Update, move_background.run_if(is_game_active))
        .add_systems(Update, move_ground.run_if(is_game_active))
        .add_systems(Update, animate_bird.run_if(is_game_not_over))
        .add_systems(Update, start_game.run_if(is_game_not_active))
        .add_systems(Update, gravity.run_if(is_game_active))
        .add_systems(Update, jump.run_if(is_game_active))
        .add_systems(Update, pipes.run_if(is_game_active))
        .add_systems(Update, score.run_if(is_game_active))
        .add_systems(Update, render_score.run_if(is_game_active))
        .run();
}
