#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use rand::{rngs::SmallRng, RngCore, SeedableRng};
use usrlib::{
    self,
    gameengine::{
        color::{Color, BLACK, ORANGE, WHITE},
        draw_functions,
        gameframelayer::GameFrameLayer,
        position::Position,
        velocity::Velocity,
    },
    gprintln,
    graphix::picturepainting::pictures::frame::Frame,
    kernel::{
        shell::shell_handler::{activate_shell, clear_screen, deactivate_shell, get_screen_size},
        syscall::keyboard::{get_new_key_event, KeyEvent::NoEvent},
    },
    kprintln,
};

const SPIELFELDGROESSE: (u32, u32) = (300, 200);

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Shell deaktivieren
    deactivate_shell();

    // Bildschirm aufräumen
    clear_screen(false);

    // Bildschirmgröße speichern
    let (screen_height, screen_width) = get_screen_size();

    gprintln!("Du kanns jetzt richtig ein Keyboard benutzten. \"q\" zum beenden");

    // Zufallszahl
    let mut small_rng = SmallRng::seed_from_u64(123456789);

    // Erstmal Gameframe zusammenbauen
    let mut game_layers: [GameFrameLayer; 3] = [
        GameFrameLayer::new(SPIELFELDGROESSE.0 as usize, SPIELFELDGROESSE.1 as usize), // Layer 0 ist das Painting Layer
        GameFrameLayer::new(SPIELFELDGROESSE.0 as usize, SPIELFELDGROESSE.1 as usize), // Layer 1 ist das Layer für Umrandung
        GameFrameLayer::new(SPIELFELDGROESSE.0 as usize, SPIELFELDGROESSE.1 as usize), // Layer 2 ist der Spieler
    ];

    // Position zum printen der Frames
    let print_position = Position::new(
        ((screen_height / 2) as u32 - (SPIELFELDGROESSE.0 / 2)) as i32,
        ((screen_width / 2) as u32 - SPIELFELDGROESSE.1 / 2) as i32,
    );

    // Position der Turtel
    let mut current_position: Position =
        Position::new(SPIELFELDGROESSE.0 as i32 / 2, SPIELFELDGROESSE.1 as i32 / 2);

    // Vorherige Position der Turtel
    let mut last_position: Position =
        Position::new(SPIELFELDGROESSE.0 as i32 / 2, SPIELFELDGROESSE.1 as i32 / 2);

    gprintln!("Vor Fill Frame");
    // Feld Weiß füllen
    game_layers[0].fill_frame(&WHITE);

    // Umrandung machen
    game_layers[1].draw_frame_border(&ORANGE);

    // Turtel platzieren
    let mut turtel_sprite = Frame::new(11, 11);
    draw_functions::draw_cross(&BLACK, &Position::new(5, 5), &mut turtel_sprite);

    game_layers[2].draw_sprite_on_position(
        &(current_position - Position::new(5, 5)),
        &mut turtel_sprite,
    );

    // Ersten Gameframe printen
    for game_layer in game_layers.iter() {
        game_layer.paint(print_position)
    }

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
            // Bildschirm aufräumen
            clear_screen(false);

            gprintln!("App wird beendent");
            break;
        }

        // = Gameloop = //
        // Tutel bewegen
        match direction {
            'w' => {
                // In den Grenzen
                if current_position.get_y() > 10 + 5 {
                    game_layers[2].delete_sprite_on_position(
                        &(current_position - Position::new(5, 5)),
                        &mut turtel_sprite,
                    );
                    current_position.shift(Velocity::new(0f32, -10f32));
                    game_layers[2].draw_sprite_on_position(
                        &(current_position - Position::new(5, 5)),
                        &mut turtel_sprite,
                    );
                }
            }
            'a' => {
                // In den Grenzen
                if current_position.get_x() > 10 + 5 {
                    game_layers[2].delete_sprite_on_position(
                        &(current_position - Position::new(5, 5)),
                        &mut turtel_sprite,
                    );
                    current_position.shift(Velocity::new(-10f32, 0f32));
                    game_layers[2].draw_sprite_on_position(
                        &(current_position - Position::new(5, 5)),
                        &mut turtel_sprite,
                    );
                }
            }
            's' => {
                // In den Grenzen
                if (current_position.get_y() as u32) < SPIELFELDGROESSE.1 - (10 + 5) {
                    game_layers[2].delete_sprite_on_position(
                        &(current_position - Position::new(5, 5)),
                        &mut turtel_sprite,
                    );
                    current_position.shift(Velocity::new(0f32, 10f32));
                    game_layers[2].draw_sprite_on_position(
                        &(current_position - Position::new(5, 5)),
                        &mut turtel_sprite,
                    );
                }
            }
            'd' => {
                // In den Grenzen
                if (current_position.get_x() as u32) < SPIELFELDGROESSE.0 - (10 + 5) {
                    game_layers[2].delete_sprite_on_position(
                        &(current_position - Position::new(5, 5)),
                        &mut turtel_sprite,
                    );
                    current_position.shift(Velocity::new(10f32, 0f32));
                    game_layers[2].draw_sprite_on_position(
                        &(current_position - Position::new(5, 5)),
                        &mut turtel_sprite,
                    );
                }
            }

            ' ' => {
                let random_color = Color::random_color();
                game_layers[0].draw_line(&last_position, &current_position, &random_color, 5);
                last_position = current_position;
            }
            'c' => {
                let rand_num = small_rng.next_u64();
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
    turtel_sprite: &mut Frame,
    field_size: &(usize, usize),
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
            // Bildschirm aufräumen
            clear_screen(false);

            gprintln!("App wird beendent");
            break;
        }

        // = Gameloop = //
        // Tutel bewegen
        match direction {
            'w' => {
                // In den Grenzen
                if current_position.get_x() > 10 + 5 {
                    game_layers[2].delete_sprite_on_position(current_position, turtel_sprite);
                    current_position.shift(Velocity::new(-10f32, 0f32));
                    game_layers[2].draw_sprite_on_position(current_position, turtel_sprite);
                }
            }
            'a' => {
                // In den Grenzen
                if current_position.get_y() > 10 + 5 {
                    game_layers[2].delete_sprite_on_position(current_position, turtel_sprite);
                    current_position.shift(Velocity::new(0f32, -10f32));
                    game_layers[2].draw_sprite_on_position(current_position, turtel_sprite);
                }
            }
            's' => {
                // In den Grenzen
                if current_position.get_y() as usize < field_size.1 - (10 + 5) {
                    game_layers[2].delete_sprite_on_position(current_position, turtel_sprite);
                    current_position.shift(Velocity::new(-10f32, 0f32));
                    game_layers[2].draw_sprite_on_position(current_position, turtel_sprite);
                }
            }
            'd' => {
                // In den Grenzen
                if current_position.get_x() as usize < field_size.get(0) - (10 + 5) {
                    game_layers[2].delete_sprite_on_position(current_position, turtel_sprite);
                    current_position.shift(Velocity::new(-10f32, 0f32));
                    game_layers[2].draw_sprite_on_position(current_position, turtel_sprite);
                }
            }

            ' ' => {
                let rand_num = rand.next_u64();
                //let random_color = ((rand_num & 0xff_ff_ff) << 8) | 0xFF;
                //kprintln!("Random color: {:#x}", random_color);
                game_layers[0].draw_line(last_position, current_position, &GREEN, 5);
                last_position = current_position;
            }
            'c' => {
                let rand_num = rand.next_u64();
                let random_color = ((rand_num & 0xff_ff_ff) << 8) | 0xFF;
                draw_circle(
                    (rand_num % 50) as u32,
                    random_color as u32,
                    current_position,
                    &mut boardframe,
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
