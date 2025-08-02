use bevy::prelude::*;
use rand::Rng;
use crate::prefs::*;

pub fn random_pipe_position() -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let bottom = -rng.gen_range(200. .. 400.);
    (bottom, bottom + 800.)
}

pub fn align(sprite_width:f32, sprite_height:f32,normalized_x:f32,normalized_y:f32,scale:f32,layer:f32) -> Transform {
    let ratio = scale*WINDOW_WIDTH/sprite_width;
    Transform {
        translation: Vec3::new(normalized_x*(WINDOW_WIDTH-sprite_width*(ratio))/2.,normalized_y*(WINDOW_HEIGHT-sprite_height*(ratio))/2.,layer),
        scale: Vec3::new(ratio,ratio,1.),
        ..Default::default()
    }
}