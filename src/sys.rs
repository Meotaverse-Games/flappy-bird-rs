use bevy::prelude::*;
use crate::components::*;

pub fn animate_tap(time: Res<Time>, mut query: Query<(&mut Tap, &mut TextureAtlas)>) {
    for (mut bird, mut texture_atlas) in query.iter_mut() {
        let delta = time.delta();
        bird.timer.tick(delta);
        if bird.timer.finished() {
            texture_atlas.index = if texture_atlas.index == 1 { 0 } else { 1 };
        }
    }
}

pub fn move_background(time: Res<Time>, mut query: Query<&mut Transform, With<Background>>) {
    let mut background_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();
    let delta_x = 20. * delta;
    background_transform.translation.x -= delta_x;
    if background_transform.translation.x < -276.*2. {
        background_transform.translation.x = 0.;
    }
}
pub fn move_ground(time: Res<Time>, mut query: Query<&mut Transform, With<Ground>>) {
    let mut ground_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();
    let delta_x = 100. * delta;
    ground_transform.translation.x -= delta_x;
    if ground_transform.translation.x < -276.*2. {
        ground_transform.translation.x = 0.;
    }
}

pub fn animate_bird(time: Res<Time>, mut query: Query<(&mut Bird, &mut TextureAtlas)>) {
    for (mut bird, mut texture_atlas) in query.iter_mut() {
        let delta = time.delta();
        bird.timer.tick(delta);
        if bird.timer.finished() {
            texture_atlas.index = if texture_atlas.index == 2 {
                0
            } else {
                texture_atlas.index + 1
            };
        }
    }
}