use alloc::string::String;
use usrlib::gameengine::color::WHITE;
use usrlib::gameengine::gameobject::{GameObject, GameObjectFactory};
use usrlib::gameengine::position::Position;
use usrlib::gameengine::velocity::Velocity;
use usrlib::graphix::picturepainting::pictures::frame::Frame;

pub fn construct_enemy_object(field_size: (usize, usize)) -> GameObject {
    // Sprite für Spieler erzeugen
    let mut player_sprite = Frame::new(10, 70);
    player_sprite.fill_frame(&WHITE);

    return GameObjectFactory::new()
        .set_name(String::from("Enemy"))
        .set_position(&Position::new((field_size.0 - 31) as i32, 40))
        .set_rectangle_collider(20, 70)
        .set_sprite(player_sprite)
        .create();
}

pub fn enemy_control_tick(enemy: &mut GameObject, ball: &GameObject) {
    // Position des Balls holen
    let ball_position = ball.get_position();

    // Ball ist höher
    if ball_position.get_y() < enemy.get_position().get_y() -5 {
        enemy.set_new_velocity(&Velocity::new(0f32, -10f32));
        return;
    }

    // Ball ist tiefer
    if ball_position.get_y() > enemy.get_position().get_y() +75 {
        enemy.set_new_velocity(&Velocity::new(0f32, 10f32));
        return;
    }

    // Genau richtig
    enemy.set_new_velocity(&Velocity::new(0f32, 0f32));
}
