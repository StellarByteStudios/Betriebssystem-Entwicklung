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

use crate::startup::{build_field, construct_ball_object, construct_border_objects};

const SPIELFELDGROESSE: (usize, usize) = (800, 500);

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

    gprintln!("Objekte angelegt");

    // Ball in 1 Layer packen
    ball.print_on_game_layer(&mut game_layers[1]);

    // Ersten Gameframe printen
    //for game_layer in game_layers.iter() {
    //    game_layer.paint(&game_print_position)
    //}
    GameFrameLayer::paint_layers(&game_layers, &game_print_position);

    // Haupt Gameloop
    //gprintln!("Loop geht los");

    loop {
        // Key holen
        let keyevent = get_new_key_event();

        // Nichts wurde gedrückt
        if keyevent != NoEvent {
            let direction = keyevent.as_char();

            // Exit
            if direction == 'q' {
                break;
            }
        }

        // Kolisionen Checken
        for border in borders.iter() {
            // Kolision holen
            let colision = ball.check_collision(border);

            // Wenn es eine gab richtung ändern
            if colision.is_some() {
                let direction = colision.unwrap();

                //kprintln!("Collision: {:?}", border);

                match direction.as_str() {
                    "North" => {
                        let mut new_velocity = ball.get_velocity();
                        new_velocity.bounce_on(Up);
                        ball.set_new_velocity(&new_velocity);
                        kprintln!("Ball bounce up");
                    }
                    "South" => {
                        let mut new_velocity = ball.get_velocity();
                        new_velocity.bounce_on(Down);
                        ball.set_new_velocity(&new_velocity);
                        kprintln!("Ball bounce Down");
                    }

                    "East" => {
                        let mut new_velocity = ball.get_velocity();
                        new_velocity.bounce_on(Left);
                        ball.set_new_velocity(&new_velocity);
                        kprintln!("Ball bounce Left");
                    }
                    "West" => {
                        let mut new_velocity = ball.get_velocity();
                        new_velocity.bounce_on(Right);
                        ball.set_new_velocity(&new_velocity);
                        kprintln!("Ball bounce Right");
                    }
                    _ => {} // Alle anderen ignorieren
                }
            }
        }

        // Bewegung des Balls
        ball.visual_tick(&mut game_layers[1]);

        // Ersten Gameframe printen
        //for game_layer in game_layers.iter() {
        //    game_layer.paint(&game_print_position)
        //}
        GameFrameLayer::paint_layers(&game_layers, &game_print_position);
    }

    // Bildschirm aufräumen
    clear_screen(false);
    gprintln!("App wird beendent");

    // Shell wieder freigeben
    activate_shell();
}

/*
fn game_loop(
    game_layers: &mut [GameFrameLayer],
    rand: &mut SmallRng,
    current_position: &mut Position,
    last_position: &mut Position,
    print_position: &Position,
    turtel_sprite: &Frame,
    field_size: &(u32, u32),
) {
    loop {
        // Key holen
        let keyevent = get_new_key_event();

        // Nichts wurde gedrückt
        if keyevent == NoEvent {
            continue;
        }

        let direction = keyevent.as_char();

        // Exit
        if direction == 'q' {
            return;
        }

        // = Gameloop = //
        // Tutel bewegen
        match direction {
            'w' => {
                // In den Grenzen
                if current_position.get_y() > 10 + 5 {
                    game_layers[2].delete_sprite_on_position(
                        &(current_position.clone() - Position::new(5, 5)),
                        turtel_sprite,
                    );
                    current_position.shift(Velocity::new(0f32, -10f32));
                    game_layers[2].draw_sprite_on_position(
                        &(current_position.clone() - Position::new(5, 5)),
                        turtel_sprite,
                    );
                }
            }
            'a' => {
                // In den Grenzen
                if current_position.get_x() > 10 + 5 {
                    game_layers[2].delete_sprite_on_position(
                        &(current_position.clone() - Position::new(5, 5)),
                        turtel_sprite,
                    );
                    current_position.shift(Velocity::new(-10f32, 0f32));
                    game_layers[2].draw_sprite_on_position(
                        &(current_position.clone() - Position::new(5, 5)),
                        turtel_sprite,
                    );
                }
            }
            's' => {
                // In den Grenzen
                if (current_position.get_y() as u32) < SPIELFELDGROESSE.1 - (10 + 5) {
                    game_layers[2].delete_sprite_on_position(
                        &(current_position.clone() - Position::new(5, 5)),
                        turtel_sprite,
                    );
                    current_position.shift(Velocity::new(0f32, 10f32));
                    game_layers[2].draw_sprite_on_position(
                        &(current_position.clone() - Position::new(5, 5)),
                        turtel_sprite,
                    );
                }
            }
            'd' => {
                // In den Grenzen
                if (current_position.get_x() as u32) < SPIELFELDGROESSE.0 - (10 + 5) {
                    game_layers[2].delete_sprite_on_position(
                        &(current_position.clone() - Position::new(5, 5)),
                        turtel_sprite,
                    );
                    current_position.shift(Velocity::new(10f32, 0f32));
                    game_layers[2].draw_sprite_on_position(
                        &(current_position.clone() - Position::new(5, 5)),
                        turtel_sprite,
                    );
                }
            }

            ' ' => {
                let random_color = Color::random_color();
                game_layers[0].draw_line(&last_position, &current_position, &random_color, 5);
                *last_position = current_position.clone();
            }
            'c' => {
                let rand_num = rand.next_u64();
                let random_color = Color::random_color();
                game_layers[0].draw_circle(
                    &current_position,
                    (rand_num % 50) as u32,
                    &random_color,
                );
            }

            _ => {
                // Falsche Richtung. Es passiert garnix
                kprintln!("Invalid direction: {}", direction);
            }
        }
        // Gameframe aktuallisieren
        for game_layer in game_layers.iter() {
            game_layer.paint(print_position)
        }
    }
}
*/
