mod prefs;
mod components;

use bevy::prelude::*;
use crate::{
    components::{Background, Bird, BottomPipe, GameOver, GetReady, Ground, Score, Tap}, prefs::{WINDOW_HEIGHT, WINDOW_WIDTH}
};

fn align(sprite_width:f32, sprite_height:f32,normalized_x:f32,normalized_y:f32,scale:f32,layer:f32) -> Transform {
    let ratio = scale*WINDOW_WIDTH/sprite_width;
    Transform {
        translation: Vec3::new(normalized_x*(WINDOW_WIDTH-sprite_width*(ratio))/2.,normalized_y*(WINDOW_HEIGHT-sprite_height*(ratio))/2.,layer),
        scale: Vec3::new(ratio,ratio,1.),
        ..Default::default()
    }
}

fn setup(mut commands:Commands,
    asset_server:Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let number_layout = TextureAtlasLayout::from_grid(Vec2::new(24., 36.), 1, 10, None, None);
    let number_atlas_layout:Handle<TextureAtlasLayout> = texture_atlas_layouts.add(number_layout);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        texture: asset_server.load("background.png"),
        transform: align(276., 228.,0.,-1.,1.,0.),
        ..default()
    }, Background));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("ground.png"),
        transform: align(276., 112.,0.,-1.,1.,1.),
        ..default()
    }, Ground));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("getready.png"),
        transform: align(174., 160.,0.,0.1,0.65,3.),
        ..default()
    }, GetReady));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("tap-1.png"),
        transform: align(174., 160.,0.,-0.5,0.65,3.),
        ..default()
    }, Tap));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("gameover.png"),
        transform: align(174., 160.,0.,0.1,0.65,3.),
        visibility:Visibility::Hidden,
        ..default()
    }, GameOver));
    for i in 0..3 {
        commands.spawn((SpriteBundle {
            texture: asset_server.load("numbers.png"),
            transform: align(24., 32.,-0.95+(0.12*(i as f32)),0.95,0.05,3.),
            ..default()
        }, TextureAtlas {
            index: 0,
            layout: number_atlas_layout.clone()
        },Score));
    }
    commands.spawn((SpriteBundle {
        texture: asset_server.load("bird.png"),
        transform: Transform {
            translation:Vec3::new(-180.,240.,2.),
            scale: Vec3::new(2.,2.,1.),
            ..Default::default()
        },
        ..default()
    }, TextureAtlas {
        index: 1,
        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(Vec2::new(34., 24.),3,1,None,None,)),
    },Bird));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pipe.png"),
        transform: Transform {
            translation:Vec3::new(220.,-200.,0.5),
            scale: Vec3::new(1.5,1.5,1.),
            ..Default::default()
        },
        ..default()
    },BottomPipe));
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.188, 0.753, 0.875)))
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
        .run();
}
