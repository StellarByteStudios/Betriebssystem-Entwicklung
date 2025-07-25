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
    utility::mathadditions::math::abs,
};

use crate::{
    score::Score,
    sounds::{play_point_scored, play_simple_collision},
    BALLSPEED, PLAYERSIZE,
};

pub fn construct_ball_object(field_size: (usize, usize)) -> GameObject {
    // Richtung des Geschwindikeitsvektors zufällig (Gewicht seitlich)
    let mut random = SmallRng::seed_from_u64(usr_get_systime());
    let x = random.gen_range(-1.0..=1.0);
    let y = random.gen_range(-1.0..=1.0) / 2f32;
    let direction = Velocity::new(x, y);
    let direction_normalized = direction.normalize() * BALLSPEED as u32;

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
                    let old_velocity = ball.get_velocity();
                    let new_y_speed = old_velocity.get_y().abs();
                    ball.set_new_velocity(&Velocity::new(old_velocity.get_x(), new_y_speed));
                    kprintln!("Ball bounce up");
                    play_simple_collision();
                }
                "South" => {
                    let old_velocity = ball.get_velocity();
                    let new_y_speed = old_velocity.get_y().abs() * -1f32;
                    ball.set_new_velocity(&Velocity::new(old_velocity.get_x(), new_y_speed));
                    kprintln!("Ball bounce Down");
                    play_simple_collision();
                }

                "East" => {
                    kprintln!("Score for Player");
                    play_point_scored();
                    score.score_player();
                    reset_ball(ball, game_frame_layer, field_size);
                }
                "West" => {
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
        // Geschwindigkeit umdrehen
        let mut ball_velocity = ball.get_velocity();
        let mut x_velocity = ball_velocity.get_x().abs(); // Spielerfall

        if player.get_name() == "Enemy" {
            x_velocity = ball_velocity.get_x().abs() * -1f32;
        }

        // Berechne Abstand von Spielerzentrum auf Y-Achse
        let ball_mid_y = ball.get_position().get_y() + 5; // 5 = halbe Ballhöhe
        let player_mid_y = player.get_position().get_y() + (PLAYERSIZE / 2) as i32;

        let axis_diff = (ball_mid_y - player_mid_y) as f32;
        let max_offset = (PLAYERSIZE / 2) as f32;

        // Y-Richtung anpassen (je nach Position des Aufpralls)
        let normalized = (axis_diff / max_offset).clamp(-1.0, 1.0);
        let y_velocity = normalized * x_velocity.abs();

        // Optional: normieren, um gleichbleibende Geschwindigkeit zu garantieren
        let new_velocity = Velocity::new(x_velocity, y_velocity).normalize() * BALLSPEED as u32;

        ball.set_new_velocity(&new_velocity);
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
    //let x = random.gen_range(-1.0..=1.0);
    let mut x = 0.0;
    while abs(x) < 0.5 {
        x = random.gen_range(-1.0..=1.0);
    }
    let y = random.gen_range(-1.0..=1.0);
    let direction = Velocity::new(x, y);
    let direction_normalized = direction.normalize() * BALLSPEED as u32;
    ball.set_new_velocity(&direction_normalized);
    ball.visual_move(
        game_frame_layer,
        &Position::new((field_size.0 / 2) as i32, (field_size.1 / 2) as i32),
    )
}
