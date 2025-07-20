use alloc::string::String;

use rand::{prelude::SmallRng, Rng, SeedableRng};
use usrlib::{
    gameengine::{
        color::YELLOW,
        directions::Direction::{Down, Left, Right, Up},
        gameframelayer::GameFrameLayer,
        gameobject::{GameObject, GameObjectFactory},
        position::Position,
        velocity::Velocity,
    },
    graphix::picturepainting::pictures::frame::Frame,
    kernel::syscall::user_api::usr_get_systime,
    kprintln,
};

use crate::{
    score::Score,
    sounds::{play_point_scored, play_simple_collision},
};

pub fn construct_ball_object(field_size: (usize, usize)) -> GameObject {
    // Richtung des Geschwindikeitsvektors zufällig (Gewicht seitlich)
    let mut random = SmallRng::seed_from_u64(usr_get_systime());
    let x = random.gen_range(-1.0..=1.0);
    let y = random.gen_range(-1.0..=1.0) / 2f32;
    let direction = Velocity::new(x, y as f32);
    let direction_normalized = direction.normalize() * 15u32;

    // Ball Frame erzeugen
    let mut ball_sprite = Frame::new(10, 10);
    ball_sprite.fill_frame(&YELLOW);

    return GameObjectFactory::new()
        .set_name(String::from("Ball"))
        .set_position(&Position::new(
            (field_size.0 / 2) as i32,
            (field_size.1 / 2) as i32,
        ))
        .set_rectangle_collider(10, 10)
        .set_velocity(direction_normalized)
        .set_sprite(ball_sprite)
        .create();
}

pub fn check_ball_collision_with_borders(
    ball: &mut GameObject,
    borders: &[GameObject],
    score: &mut Score,
    game_frame_layer: &mut GameFrameLayer,
    field_size: (usize, usize),
) {
    for border in borders.iter() {
        // Kolision holen
        let colision = ball.check_collision(border);

        // Wenn es eine gab richtung ändern
        if colision.is_some() {
            let partner = colision.unwrap();

            match partner.as_str() {
                "North" => {
                    let mut new_velocity = ball.get_velocity();
                    new_velocity.bounce_on(Up);
                    ball.set_new_velocity(&new_velocity);
                    kprintln!("Ball bounce up");
                    play_simple_collision();
                }
                "South" => {
                    let mut new_velocity = ball.get_velocity();
                    new_velocity.bounce_on(Down);
                    ball.set_new_velocity(&new_velocity);
                    kprintln!("Ball bounce Down");
                    play_simple_collision();
                }

                "East" => {
                    let mut new_velocity = ball.get_velocity();
                    new_velocity.bounce_on(Left);
                    ball.set_new_velocity(&new_velocity);
                    kprintln!("Score for Player");
                    play_point_scored();
                    score.score_player();
                    reset_ball(ball, game_frame_layer, field_size);
                }
                "West" => {
                    let mut new_velocity = ball.get_velocity();
                    new_velocity.bounce_on(Right);
                    ball.set_new_velocity(&new_velocity);
                    kprintln!("Score for Enemy");
                    play_point_scored();
                    score.score_enemy();
                    reset_ball(ball, game_frame_layer, field_size);
                }
                _ => {} // Alle anderen ignorieren
            }
        }
    }
}

pub fn check_ball_collision_with_player(ball: &mut GameObject, player: &GameObject) {
    let player_colision = ball.check_collision(&player);
    if player_colision.is_some() {
        let mut new_velocity = ball.get_velocity();
        new_velocity.bounce_on(Left);
        ball.set_new_velocity(&new_velocity);
        kprintln!("Ball Hit Player");
        play_simple_collision()
    }
}

pub fn reset_ball(
    ball: &mut GameObject,
    game_frame_layer: &mut GameFrameLayer,
    field_size: (usize, usize),
) {
    // Richtung des Geschwindikeitsvektors zufällig (Gewicht seitlich)
    let mut random = SmallRng::seed_from_u64(usr_get_systime());
    let x = random.gen_range(-1.0..=1.0);
    let y = random.gen_range(-1.0..=1.0) / 2f32;
    let direction = Velocity::new(x, y as f32);
    let direction_normalized = direction.normalize() * 15u32;

    ball.set_new_velocity(&direction_normalized);
    ball.visual_move(
        game_frame_layer,
        &Position::new((field_size.0 / 2) as i32, (field_size.1 / 2) as i32),
    )
}
