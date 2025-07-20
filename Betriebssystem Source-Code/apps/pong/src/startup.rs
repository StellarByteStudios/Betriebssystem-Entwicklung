use alloc::string::String;

use usrlib::{
    gameengine::{
        color::{BLACK, WHITE},
        gameframelayer::GameFrameLayer,
        gameobject::{GameObject, GameObjectFactory},
        position::Position,
    },
    gprintln,
    graphix::picturepainting::pictures::frame::Frame,
    kernel::shell::shell_handler::{clear_screen, deactivate_shell, get_screen_size},
};

use crate::PLAYERSIZE;

const SEED: u64 = 42;

pub fn build_field(field_size: (usize, usize)) -> ([GameFrameLayer; 2], Position) {
    // Shell deaktivieren
    deactivate_shell();

    // Bildschirm aufräumen
    clear_screen(false);

    // Bildschirmgröße speichern
    let (screen_width, screen_height) = get_screen_size();

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
        .set_rectangle_collider(field_size.0, 15)
        .create();

    let south_border = GameObjectFactory::new()
        .set_name(String::from("South"))
        .set_position(&Position::new(0, (field_size.1 - 15) as i32))
        .set_rectangle_collider(field_size.0, 15)
        .create();

    let east_border = GameObjectFactory::new()
        .set_name(String::from("East"))
        .set_position(&Position::new((field_size.0 - 15) as i32, 0))
        .set_rectangle_collider(15, field_size.1)
        .create();

    let west_border = GameObjectFactory::new()
        .set_name(String::from("West"))
        .set_position(&Position::new(0, 0))
        .set_rectangle_collider(15, field_size.1)
        .create();

    return [north_border, south_border, east_border, west_border];
}

pub fn construct_player_object() -> GameObject {
    // Sprite für Spieler erzeugen
    let mut player_sprite = Frame::new(10, PLAYERSIZE as u32);
    player_sprite.fill_frame(&WHITE);

    return GameObjectFactory::new()
        .set_name(String::from("Player 1"))
        .set_position(&Position::new(20, 40))
        .set_rectangle_collider(20, PLAYERSIZE)
        .set_sprite(player_sprite)
        .create();
}
