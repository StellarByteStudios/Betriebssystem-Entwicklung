#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

mod startup;

use alloc::{vec, vec::Vec};

use rand::{RngCore, SeedableRng};
use usrlib::{
    self,
    gameengine::{
        directions::Direction::{Down, Left, Right, Up},
        gameframelayer::GameFrameLayer,
        position::Position,
    },
    gprintln,
    kernel::{
        shell::shell_handler::{activate_shell, clear_screen},
        syscall::keyboard::{get_new_key_event, KeyEvent::NoEvent},
    },
    kprintln,
    utility::delay::delay,
};
use usrlib::gameengine::velocity::Velocity;
use usrlib::music::note::Note;
use usrlib::music::player::play_note;
use crate::startup::{build_field, construct_ball_object, construct_border_objects, construct_player_object};

const SPIELFELDGROESSE: (usize, usize) = (800, 500);

const BEEP: Note = Note{ frequency: 800, duration: 10};

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

    gprintln!("Objekte angelegt");

    // Ball und Spieler in 1 Layer packen
    ball.print_on_game_layer(&mut game_layers[1]);
    player.print_on_game_layer(&mut game_layers[1]);


    GameFrameLayer::paint_layers(&game_layers, &game_print_position);

    // Haupt Gameloop
    loop {
        // Key holen
        let keyevent = get_new_key_event();

        // Nichts wurde gedrückt
        if keyevent != NoEvent {
            let direction = keyevent.as_char();

            // Input verarbeiten
            match direction {
                'q' => break,
                'w' => player.set_new_velocity(&Velocity::new(0f32, -10f32)),
                's' => player.set_new_velocity(&Velocity::new(0f32, 10f32)),
                _ => kprintln!("{} invalid imput", direction) // nichts machen

            }
        }

        // Kolisionen Checken
        for border in borders.iter() {
            // Kolision holen
            let colision = ball.check_collision(border);

            // Wenn es eine gab richtung ändern
            if colision.is_some() {
                let partner = colision.unwrap();

                //kprintln!("Collision: {:?}", border);

                match partner.as_str() {
                    "North" => {
                        let mut new_velocity = ball.get_velocity();
                        new_velocity.bounce_on(Up);
                        ball.set_new_velocity(&new_velocity);
                        kprintln!("Ball bounce up");
                        play_note(BEEP);
                    }
                    "South" => {
                        let mut new_velocity = ball.get_velocity();
                        new_velocity.bounce_on(Down);
                        ball.set_new_velocity(&new_velocity);
                        kprintln!("Ball bounce Down");
                        play_note(BEEP);
                    }

                    "East" => {
                        let mut new_velocity = ball.get_velocity();
                        new_velocity.bounce_on(Left);
                        ball.set_new_velocity(&new_velocity);
                        kprintln!("Ball bounce Left");
                        play_note(BEEP);
                    }
                    "West" => {
                        let mut new_velocity = ball.get_velocity();
                        new_velocity.bounce_on(Right);
                        ball.set_new_velocity(&new_velocity);
                        kprintln!("Ball bounce Right");
                        play_note(BEEP);
                    }
                    _ => {} // Alle anderen ignorieren
                }
            }
        }

        // Kollision mit spieler
        let player_colision = ball.check_collision(&player);
        if player_colision.is_some() {
            let mut new_velocity = ball.get_velocity();
            new_velocity.bounce_on(Left);
            ball.set_new_velocity(&new_velocity);
            kprintln!("Ball Hit Player");
            play_note(BEEP);
        }

        // Bewegung des Balls
        ball.visual_tick(&mut game_layers[1]);
        
        // Bewegung des Spielers
        // ist er in den Grenzen
        if !(player.get_position().get_y() > 20 && player.get_position().get_y() < (SPIELFELDGROESSE.1 - 80) as i32) {
            let mut new_velocity = player.get_velocity();
            new_velocity.bounce_on(Up);
            player.set_new_velocity(&new_velocity);
        }

        player.visual_tick(&mut game_layers[1]);

        GameFrameLayer::paint_layers(&game_layers, &game_print_position);
    }

    // Bildschirm aufräumen
    clear_screen(false);
    gprintln!("App wird beendent");

    // Shell wieder freigeben
    activate_shell();
}