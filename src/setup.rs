use bevy::prelude::*;
use crate::{
    components::{Background, Bird, BottomPipe, GameOver, GetReady, Ground, Score, Tap, TopPipe},
    prefs::WINDOW_WIDTH,
    utils::{align,random_pipe_position}
};

pub fn setup(mut commands:Commands,
    asset_server:Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let number_layout = TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
    let number_atlas_layout:Handle<TextureAtlasLayout> = texture_atlas_layouts.add(number_layout);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((SpriteBundle {
        texture: asset_server.load("background.png"),
        sprite: Sprite{
            custom_size: Some(Vec2::new(WINDOW_WIDTH+276.*4.,228.)),
            ..default()
        },
        transform: align(276., 228.,0.,-1.,1.,0.),
        ..default()
    },ImageScaleMode::Tiled {
        tile_x:true,tile_y:false,stretch_value:1.
    },
     Background));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("ground.png"),
        sprite: Sprite{
            custom_size: Some(Vec2::new(WINDOW_WIDTH+276.*4.,112.)),
            ..default()
        },
        transform: align(276., 112.,0.,-1.,1.,1.),
        ..default()
    },ImageScaleMode::Tiled {
        tile_x:true,tile_y:false,stretch_value:1.
    }, Ground));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("getready.png"),
        transform: align(174., 160.,0.,0.1,0.65,3.),
        visibility: Visibility::Visible,
        ..default()
    }, GetReady));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("tap.png"),
        transform: align(174., 160.,0.,-0.5,0.65,3.),
        visibility:Visibility::Visible,
        ..default()
    },TextureAtlas {
        index: 0,
        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::new(118, 36),2,1,None,None,)),
    },
    Tap {
        timer:Timer::from_seconds(0.2, TimerMode::Repeating)
    }));
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
        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::new(34, 24),3,1,None,None,)),
    },Bird {
        timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        velocity: 0.
    }));
    for i in 0..5 {
        let dist = i as f32 * 300.;
        let (bottom,top) = random_pipe_position();
        commands.spawn((SpriteBundle {
            texture: asset_server.load("pipe.png"),
            transform: Transform {
                translation:Vec3::new(220.+dist,bottom,0.5),
                scale: Vec3::new(1.5,1.5,1.),
                ..Default::default()
            },
            ..default()
        },BottomPipe));
        commands.spawn((SpriteBundle {
            texture: asset_server.load("pipe.png"),
            transform: Transform {
                translation:Vec3::new(220.+dist,top,0.5),
                scale: Vec3::new(1.5,1.5,1.),
                rotation:Quat::from_rotation_z(std::f32::consts::PI),
                ..Default::default()
            },
            ..default()
        },TopPipe { passed:false }));
    }
}