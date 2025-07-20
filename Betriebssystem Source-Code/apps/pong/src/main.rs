#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

mod ball;
mod score;
mod sounds;
mod startup;
mod enemy;

use ball::construct_ball_object;
use rand::{RngCore, SeedableRng};
use usrlib::{
    self,
    gameengine::{
        directions::Direction::Up, gameframelayer::GameFrameLayer, gameobject::GameObject,
        position::Position, velocity::Velocity,
    },
    gprintln,
    kernel::{
        shell::shell_handler::{activate_shell, clear_screen},
        syscall::keyboard::{get_new_key_event, KeyEvent::NoEvent},
    },
    kprintln,
};

use crate::{
    ball::{check_ball_collision_with_borders, check_ball_collision_with_player},
    score::Score,
    startup::{build_field, construct_border_objects, construct_player_object},
};
use crate::enemy::{construct_enemy_object, enemy_control_tick};

const SPIELFELDGROESSE: (usize, usize) = (650, 400);

pub const PLAYERSIZE: usize = 70;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Spielfeld initialisieren
    let (mut game_layers, game_print_position): ([GameFrameLayer; 2], Position) =
        build_field(SPIELFELDGROESSE);
    gprintln!("Layer angelegt");

    // Ränder holen
    let borders = construct_border_objects(SPIELFELDGROESSE);

    // Ball holen
    let mut ball = construct_ball_object(SPIELFELDGROESSE);

    // Spieler Holen
    let mut player = construct_player_object();

    // Gegner holen
    let mut enemy = construct_enemy_object(SPIELFELDGROESSE);

    // Leeren Score
    let mut play_score = Score::new();

    gprintln!("Objekte angelegt");

    // Ball und Spieler in 1 Layer packen
    ball.print_on_game_layer(&mut game_layers[1]);
    player.print_on_game_layer(&mut game_layers[1]);
    enemy.print_on_game_layer(&mut game_layers[1]);

    GameFrameLayer::paint_layers(&game_layers, &game_print_position);

    // Haupt Gameloop
    gameloop(
        &mut player,
        &mut ball,
        &mut enemy,
        &borders,
        &game_print_position,
        &mut game_layers,
        &mut play_score,
    );

    // Bildschirm aufräumen
    clear_screen(false);
    gprintln!("App wird beendent");

    // Shell wieder freigeben
    activate_shell();
}

fn gameloop(
    player: &mut GameObject,
    ball: &mut GameObject,
    enemy: &mut GameObject,
    borders: &[GameObject],
    game_print_position: &Position,
    game_layers: &mut [GameFrameLayer],
    play_score: &mut Score,
) {
    loop {
        // Eingabe verarbeiten
        let end_programm = check_input(player);
        if end_programm {
            return;
        }

        // Kolisionen Checken
        check_ball_collision_with_borders(
            ball,
            &borders,
            play_score,
            &mut game_layers[1],
            SPIELFELDGROESSE,
        );

        // Kollision mit Spieler
        check_ball_collision_with_player(ball, &player);
        check_ball_collision_with_player(ball, &enemy);

        // Bewegung des Balls
        ball.visual_tick(&mut game_layers[1]);

        // Bewegung des Spielers
        player_bounds_check(player);
        player.visual_tick(&mut game_layers[1]);

        // Bewegung des Gegners
        player_bounds_check(enemy);
        enemy_control_tick(enemy, ball);
        enemy.visual_tick(&mut game_layers[1]);

        // Feld ausgeben
        GameFrameLayer::paint_layers(&game_layers, &game_print_position);

        // Aktuellen Score ausgeben
        play_score.print_score(&Position::new(52, 14))
    }
}

// Gibt true zurück wenn das Spiel beendet werden soll
fn check_input(player: &mut GameObject) -> bool {
    // Key holen
    let keyevent = get_new_key_event();

    // Nichts wurde gedrückt
    if keyevent != NoEvent {
        let direction = keyevent.as_char();

        // Input verarbeiten
        match direction {
            'q' => return true,
            'w' => player.set_new_velocity(&Velocity::new(0f32, -10f32)),
            's' => player.set_new_velocity(&Velocity::new(0f32, 10f32)),
            _ => kprintln!("{} invalid imput", direction), // nichts machen
        }
    }

    return false;
}

fn player_bounds_check(player: &mut GameObject) {
    // Grenze nach oben
    if player.get_position().get_y() <=10 {
        player.set_new_velocity(&Velocity::new(0f32, 10f32));
        return;
    }
    // Genze nach unten
    if player.get_position().get_y() >= (SPIELFELDGROESSE.1 - PLAYERSIZE - 10) as i32 {
        player.set_new_velocity(&Velocity::new(0f32, -10f32));
    }
}
