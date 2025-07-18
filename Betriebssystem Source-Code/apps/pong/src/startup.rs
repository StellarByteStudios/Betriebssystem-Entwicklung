use alloc::string::String;

use rand::{rngs::SmallRng, RngCore, SeedableRng};
use usrlib::{
    gameengine::{
        color::{BLACK, GREEN, WHITE, YELLOW},
        gameframelayer::GameFrameLayer,
        gameobject::{GameObject, GameObjectFactory},
        position::Position,
        velocity::Velocity,
    },
    gprintln,
    graphix::picturepainting::pictures::frame::Frame,
    kernel::{
        shell::shell_handler::{clear_screen, deactivate_shell, get_screen_size},
        syscall::user_api::usr_get_systime,
    },
};

const SEED: u64 = 42;

pub fn build_field(field_size: (usize, usize)) -> ([GameFrameLayer; 2], Position) {
    // Shell deaktivieren
    deactivate_shell();

    // Bildschirm aufräumen
    clear_screen(false);

    // Bildschirmgröße speichern
    let (screen_width, screen_height) = get_screen_size();

    gprintln!("Du kanns jetzt richtig ein Keyboard benutzten. \"q\" zum beenden");

    // Erstmal Gameframe zusammenbauen
    let mut game_layers: [GameFrameLayer; 2] = [
        GameFrameLayer::new(field_size.0, field_size.1), // Layer 0 ist das Hintergrund Layer
        GameFrameLayer::new(field_size.0, field_size.1), // Layer 1 ist Spieler Layer
    ];

    // Position zum printen der Frames
    let print_position = Position::new(
        ((screen_width as usize / 2) - (field_size.0 / 2)) as i32,
        ((screen_height as usize / 2) - field_size.1 / 2) as i32,
    );

    // Feld Schwarz füllen
    game_layers[0].fill_frame(&BLACK);

    // Umrandung machen
    game_layers[0].draw_frame_border(&WHITE);

    // Spielfeldlinie
    game_layers[0].draw_line(
        &Position::new((field_size.0 / 2) as i32, 0),
        &Position::new((field_size.0 / 2) as i32, (field_size.1 - 1) as i32),
        &WHITE,
        3,
    );

    return (game_layers, print_position);
}

pub fn construct_border_objects(field_size: (usize, usize)) -> [GameObject; 4] {
    let north_border = GameObjectFactory::new()
        .set_name(String::from("North"))
        .set_position(&Position::new(0, 0))
        .set_rectangle_collider(field_size.0, 20)
        .create();

    let south_border = GameObjectFactory::new()
        .set_name(String::from("South"))
        .set_position(&Position::new(0, (field_size.1 - 20) as i32))
        .set_rectangle_collider(field_size.0, 20)
        .create();

    let east_border = GameObjectFactory::new()
        .set_name(String::from("East"))
        .set_position(&Position::new((field_size.0 - 20) as i32, 0))
        .set_rectangle_collider(20, field_size.1)
        .create();

    let west_border = GameObjectFactory::new()
        .set_name(String::from("West"))
        .set_position(&Position::new(0, 0))
        .set_rectangle_collider(20, field_size.1)
        .create();

    return [north_border, south_border, east_border, west_border];
}

//pub fn construct_player_object() -> GameObject {}

pub fn construct_ball_object(field_size: (usize, usize)) -> GameObject {
    // Richtung des Geschwindikeitsvektors zufällig
    let mut random = SmallRng::seed_from_u64(usr_get_systime());
    let direction = Velocity::new(random.next_u32() as f32, random.next_u32() as f32);
    let direction_normalized = direction.normalize() * 20u32;

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


pub fn construct_player_object() -> GameObject {
    // Sprite für Spieler erzeugen
    let mut player_sprite = Frame::new(10, 70);
    player_sprite.fill_frame(&WHITE);

    return GameObjectFactory::new()
        .set_name(String::from("Player 1"))
        .set_position(&Position::new(20, 40))
        .set_rectangle_collider(20, 70)
        .set_sprite(player_sprite)
        .create();
}
