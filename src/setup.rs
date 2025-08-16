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
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite {
            image: asset_server.load("background.png"),
            custom_size: Some(Vec2::new(WINDOW_WIDTH+276.*4.,228.)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.
            },
            ..Default::default()
        },
        align(276., 228.,0.,-1.,1.,0.),
        Background
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("ground.png"),
            custom_size: Some(Vec2::new(WINDOW_WIDTH+276.*4.,112.)),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: true,
                stretch_value: 1.
            },
            ..Default::default()
        },
        align(276., 112.,0.,-1.,1.,1.),
        Ground
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("getready.png"),
            ..Default::default()
        },
        align(174., 160.,0.,0.1,0.65,3.),
        Visibility::Visible,
        GetReady
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("tap.png"),
            texture_atlas: Some(TextureAtlas {
                index:0,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::new(118, 36), 2, 1, None, None))
            }),
            ..Default::default()
        },
        align(174., 160.,0.,-0.5,0.65,3.),
        Visibility::Visible,
        Tap {
            timer:Timer::from_seconds(0.2, TimerMode::Repeating)
        }
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("gameover.png"),
            ..Default::default()
        },
        align(174., 160.,0.,0.1,0.65,3.),
        Visibility::Hidden,
        GameOver
    ));
    for i in 0..3 {
            commands.spawn((
                Sprite {
                    image: asset_server.load("numbers.png"),
                    texture_atlas: Some(TextureAtlas {
                        index: 0,
                        layout:number_atlas_layout.clone()
                    }),
                ..Default::default()
                },
                align(24., 32.,-0.95+(0.12*(i as f32)),0.95,0.05,3.),
                Score
        ));
    }
    commands.spawn((
        Sprite {
            image: asset_server.load("bird.png"),
            texture_atlas: Some(TextureAtlas {
                index:1,
                layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(UVec2::new(34, 24),3,1,None,None,))
            }),
            ..Default::default()
        },Transform {
            translation:Vec3::new(-180.,240.,2.),
            scale: Vec3::new(2.,2.,1.),
            ..Default::default()
        },
        Bird {
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
            velocity: 0.
        }
    ));
    for i in 0..5 {
        let dist = i as f32 * 300.;
        let (bottom,top) = random_pipe_position();
        commands.spawn((
            Sprite {
                image: asset_server.load("pipe.png"),
                ..Default::default()
            },
            Transform {
                translation:Vec3::new(220.+dist,bottom,0.5),
                scale: Vec3::new(1.5,1.5,1.),
                ..Default::default()
            },
            BottomPipe
        ));
        commands.spawn((
            Sprite {
                image: asset_server.load("pipe.png"),
                ..Default::default()
            },
            Transform {
                    translation:Vec3::new(220.+dist,top,0.5),
                    scale: Vec3::new(1.5,1.5,1.),
                    rotation:Quat::from_rotation_z(std::f32::consts::PI),
                    ..Default::default()
            },
            TopPipe { passed:false }
        ));
    }
}