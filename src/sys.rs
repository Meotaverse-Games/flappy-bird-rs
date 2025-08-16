use bevy::prelude::*;
use crate::components::*;
use crate::prefs::WINDOW_WIDTH;
use crate::resources::*;
use crate::utils::random_pipe_position;

pub fn animate_tap(time: Res<Time>, mut query: Query<(&mut Tap, &mut Sprite)>) {
    for (mut tap, mut sprite) in query.iter_mut() {
        let delta = time.delta();
        tap.timer.tick(delta);
        if tap.timer.finished() {
            if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
                texture_atlas.index ^= 1;
            }
        }
    }
}

pub fn move_background(time: Res<Time>, mut query: Query<&mut Transform, With<Background>>) {
    let mut background_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();
    let delta_x = 20. * delta;
    if let Ok(pos) = background_transform.as_mut() {
        pos.translation.x -= delta_x;
        if pos.translation.x < -276.*2. {
            pos.translation.x = 0.;
        }
    }
}
pub fn move_ground(time: Res<Time>, mut query: Query<&mut Transform, With<Ground>>) {
    let mut ground_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();
    let delta_x = 150. * delta;
    if let Ok(pos) = ground_transform.as_mut() {
        pos.translation.x -= delta_x;
        if pos.translation.x < -276.*2. {
            pos.translation.x = 0.;
        }
    }
}

pub fn animate_bird(time: Res<Time>, mut query: Query<(&mut Bird, &mut Sprite)>) {
    for (mut bird, mut sprite) in query.iter_mut() {
        let delta = time.delta();
        bird.timer.tick(delta);
        if bird.timer.finished() {
            if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
                texture_atlas.index = if texture_atlas.index == 2 {
                    0
                } else {
                    texture_atlas.index + 1
                };
            }
        }
    }
}

pub fn start_game(
    mut game: ResMut<Game>,
    mut tap_query: Query<&mut Visibility,(With<Tap>, Without<GetReady>)>,
    mut getready_query: Query<&mut Visibility,(With<GetReady>, Without<Tap>)>,
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    mut gameover_query: Query<&mut Visibility,(With<GameOver>,Without<GetReady>,Without<Tap>)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut top_pipe_query: Query<(&mut Transform, &mut TopPipe), (With<TopPipe>, Without<Bird>)>,
    mut bottom_pipe_query: Query<
        &mut Transform,
        (With<BottomPipe>, Without<Bird>, Without<TopPipe>),
    >
) {
    if !mouse_input.just_pressed(MouseButton::Left){
        return
    }

    game.score = 0;
    if game.state == GameState::GameOver {
        for (i, (mut transform, mut upper_pipe)) in top_pipe_query.iter_mut().enumerate() {
            let delta_x = i as f32 * 300.0 + 300.;

            upper_pipe.passed = false;
            transform.translation.x = 0.;
            transform.translation.x += delta_x;
        }

        for (i, mut transform) in bottom_pipe_query.iter_mut().enumerate() {
            let delta_x = i as f32 * 300.0 + 300.;

            transform.translation.x = 0.;
            transform.translation.x += delta_x;
        }
    };

    game.state = GameState::Active;

    if let Ok(mut gameover_visible) = gameover_query.single_mut() {
        *gameover_visible = Visibility::Hidden;
    }

    for (mut bird, mut transform) in bird_query.iter_mut() {
        bird.velocity = 0.;
        transform.translation.y = 240.;
        transform.rotation = Quat::from_rotation_z(0.);
    }

    if let Ok(mut tap_visible) = tap_query.single_mut() {
        *tap_visible = Visibility::Hidden;
    }
    if let Ok(mut getready_visible) = getready_query.single_mut() {
        *getready_visible = Visibility::Hidden;
    }
}

pub fn gravity(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut query: Query<(&mut Bird, &mut Transform)>,
    mut gameover_query: Query<&mut Visibility, With<GameOver>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (mut bird, mut transform) in query.iter_mut() {
        let delta = time.delta().as_secs_f32();
        let gravity = 9.8;
        let delta_v = gravity * 40. * delta;
        let delta_y = bird.velocity * delta;
        let new_y = (transform.translation.y + delta_y).min(440.0);

        transform.translation.y = new_y;
 
        bird.velocity -= delta_v;
        transform.translation.y += bird.velocity * delta;

        let rotation = bird.velocity / 600.;
        let max_rotation = 0.9;
        transform.rotation = Quat::from_rotation_z(rotation.max(-max_rotation).min(max_rotation));

        let col_point = 26./2.-160.;
        if transform.translation.y < col_point {
            transform.translation.y = col_point;
            transform.rotation = Quat::from_rotation_z(1.5*std::f32::consts::PI);
            bird.velocity = 0.;

            game.state = GameState::GameOver;
            if let Ok(mut gameover_visible) = gameover_query.single_mut(){
                *gameover_visible = Visibility::Visible;
            }

            commands.spawn((
                AudioPlayer::new(asset_server.load("sfx/hit.ogg")),
                PlaybackSettings::DESPAWN
            ));
        }
    }
}

pub fn jump(
    mut query: Query<&mut Bird>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mouse_input: Res<ButtonInput<MouseButton>>
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }
 
    commands.spawn((
        AudioPlayer::new(asset_server.load("sfx/wing.ogg")),
        PlaybackSettings::DESPAWN
    ));

    for mut bird in query.iter_mut() {
        bird.velocity = 220.0;
    }
}

pub fn pipes(
    time: Res<Time>,
    mut top_pipe_query: Query<(&mut TopPipe, &mut Transform)>,
    mut bottom_pipe_query: Query<(&BottomPipe, &mut Transform), Without<TopPipe>>,
    mut bird_query: Query<&Transform, (With<Bird>, Without<BottomPipe>, Without<TopPipe>)>,
    mut gameover_query: Query<&mut Visibility, With<GameOver>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut commands: Commands,
) {
    let delta = time.delta().as_secs_f32();
    let delta_x = 150. * delta;

    let utmost_right_pipe = top_pipe_query.iter()
        .max_by(|(_, a), (_, b)| a.translation.x.partial_cmp(&b.translation.x).unwrap()).
        unwrap().1.translation.x;

    let new_pipe_position = utmost_right_pipe + 300.;
    let (lower_y, upper_y) = random_pipe_position();
    let out_of_screen_x = -WINDOW_WIDTH / 2. - 52.;

    for (mut top_pipe, mut transform) in top_pipe_query.iter_mut() {
        transform.translation.x -= delta_x;

        if transform.translation.x < out_of_screen_x {
            transform.translation.x = new_pipe_position;
            transform.translation.y = upper_y;
            top_pipe.passed = false;
        }
    }

    for (_, mut transform) in bottom_pipe_query.iter_mut() {
        transform.translation.x -= delta_x;

        if transform.translation.x < out_of_screen_x {
            transform.translation.x = new_pipe_position;
            transform.translation.y = lower_y;
        }
    }

    let is_collision = |bird_transform: &Transform, pipe_transform: &Transform| -> bool {
        let bird_x = bird_transform.translation.x;
        let bird_y = bird_transform.translation.y;
        let bird_width = 34.0;
        let bird_height = 26.0;

        let pipe_x = pipe_transform.translation.x;
        let pipe_y = pipe_transform.translation.y;
        let pipe_width = 104.0;
        let pipe_height = 620.0;

        let collision_x = bird_x + bird_width / 2.0 > pipe_x - pipe_width / 2.0
            && bird_x - bird_width / 2.0 < pipe_x + pipe_width / 2.0;
        let collision_y = bird_y + bird_height / 2.0 > pipe_y - pipe_height / 2.0
            && bird_y - bird_height / 2.0 < pipe_y + pipe_height / 2.0;

        collision_x && collision_y
    };

    for bird_transform in bird_query.iter_mut() {
        let mut game_over = || {
            game.state = GameState::GameOver;
            if let Ok(mut gameover_visible) = gameover_query.single_mut() {
                *gameover_visible = Visibility::Visible;
            }

            commands.spawn((
                AudioPlayer::new(asset_server.load("sfx/hit.ogg")),
                PlaybackSettings::DESPAWN
            ));
        };

        for (_, transform) in top_pipe_query.iter_mut() {
            if is_collision(bird_transform, &transform) {
                game_over();
            }
        }

        for (_, transform) in bottom_pipe_query.iter_mut() {
            if is_collision(bird_transform, &transform) {
                game_over();
            }
        }
    }
}

 
pub fn score(
    mut game: ResMut<Game>,
    bird_query: Query<(&Bird, &Transform)>,
    mut upper_pipe_query: Query<(&mut TopPipe, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (_, bird_transform) in bird_query.iter() {
        for (mut upper_pipe, transform) in upper_pipe_query.iter_mut() {
            let passed = transform.translation.x < bird_transform.translation.x;
            let passed_state = upper_pipe.passed;
 
            if passed && !passed_state {
                game.score += 1;
                upper_pipe.passed = true;
 
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sfx/point.ogg")),
                    PlaybackSettings::DESPAWN
                ));
            }
        }
    }
}

pub fn render_score(game: Res<Game>, mut query: Query<&mut Sprite, With<Score>>) {
    let score_string = format!("{:03}", game.score); // Ensure at least 3 digits, pad with zeros
    let score_digits: Vec<usize> = score_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();
 
    for (digit, mut sprite) in score_digits.iter().zip(query.iter_mut()) {
        if let Some(texture_atlas) = sprite.texture_atlas.as_mut() {
            texture_atlas.index = *digit;
        }
    }
}