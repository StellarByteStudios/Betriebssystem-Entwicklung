#![no_std]
#![no_main]
#![allow(unused_variables)] // avoid warnings

extern crate alloc;

use alloc::vec;

use usrlib::{
    self, gprintln,
    graphix::picturepainting::{animate::Frame, paint::draw_picture},
    kernel::{
        shell::shell_handler::{activate_shell, deactivate_shell},
        syscall::user_api::usr_getlastkey,
    },
    kprintln,
};

const SPIELFELDGROESSE: (u32, u32) = (300, 200);

const RED: u32 = 0xff_00_00_00;
const GREEN: u32 = 0x00_ff_00_00;
const BLUE: u32 = 0x00_00_ff_00;
const BLACK: u32 = 0x00_00_00_00;

const WHITE: u32 = 0xff_ff_ff_00;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    // Shell deaktivieren
    deactivate_shell();

    gprintln!("Du kanns jetzt richtig ein Keyboard benutzten. \"q\" zum beenden");

    // Erstmal Gameframe zusammenbauen
    let mut gameframe: Frame = Frame {
        width: SPIELFELDGROESSE.0,
        height: SPIELFELDGROESSE.1,
        bpp: 4,
        data: vec![0u8; (SPIELFELDGROESSE.0 * SPIELFELDGROESSE.1 * 4) as usize],
    };

    // Position der Turtel
    let mut current_position = (SPIELFELDGROESSE.0/2, SPIELFELDGROESSE.1/2);

    // Feld Weiß füllen
    gameframe.data.fill(0xff);

    // Umrandung machen
    set_border(RED, &mut gameframe);

    // Turtel platzieren
    draw_turtel(BLACK, current_position, &mut gameframe);

    // Ersten Gameframe printen
    draw_picture(300, 200, &gameframe);

    kprintln!("Gameframe gemalt");

    loop {
        // Key holen
        let direction = usr_getlastkey() as u8 as char;

        // Exit
        if direction == 'q' {
            gprintln!("App wird beendent");
            break;
        }

        // = Gameloop = //
        // Tutel bewegen
        match direction {
            'w' => {
                draw_turtel(WHITE, current_position, &mut gameframe);
                current_position.1 -= 10;
                draw_turtel(BLACK, current_position, &mut gameframe);
            }
            'a' => {
                draw_turtel(WHITE, current_position, &mut gameframe);
                current_position.0 -= 10;
                draw_turtel(BLACK, current_position, &mut gameframe);
            }
            's' => {
                draw_turtel(WHITE, current_position, &mut gameframe);
                current_position.1 += 10;
                draw_turtel(BLACK, current_position, &mut gameframe);
            }
            'd' => {
                draw_turtel(WHITE, current_position, &mut gameframe);
                current_position.0 += 10;
                draw_turtel(BLACK, current_position, &mut gameframe);
            }

            ' ' => {
                draw_circle(10, GREEN, current_position, &mut gameframe);
            }

            _ => {
                // Falsche Richtung. Es passiert garnix
                kprintln!("Invalid direction: {}", direction);
            }
        }
        // Gameframe aktuallisieren
        draw_picture(300, 200, &gameframe);
    }

    // Shell wieder freigeben
    activate_shell();
}

fn set_color_on_pixel(color: u32, index: u32, frame: &mut Frame) {
    let i = (index * 4) as usize;

    frame.data[i] = (color >> 24) as u8; // R
    frame.data[i + 1] = (color >> 16) as u8; // G
    frame.data[i + 2] = (color >> 8) as u8; // B
    frame.data[i + 3] = color as u8; // A
}


fn draw_circle(radius: u32, color: u32, position: (u32, u32), frame: &mut Frame) {
    let (cx, cy) = position;

    // quadratischer Bereich um den Kreis ablaufen
    for dy in -(radius as i32)..=(radius as i32) {
        for dx in -(radius as i32)..=(radius as i32) {
            let x = cx as i32 + dx;
            let y = cy as i32 + dy;

            // Abstand zum Mittelpunkt berechnen (Pythagoras)
            if dx*dx + dy*dy <= (radius * radius) as i32 {
                // Nur zeichnen, wenn (x, y) im Bild liegt
                if x >= 0 && y >= 0 && (x as u32) < frame.width && (y as u32) < frame.height {
                    let index = xy_to_index(x as u32, y as u32);
                    set_color_on_pixel(color, index, frame);
                }
            }
        }
    }
}

fn draw_turtel(color: u32, position: (u32, u32), frame: &mut Frame) {
    // Strich von oben nach unten position.0 + (frame.width * (i - 3)) + position.1 -1
    for i in 0u32..11{
        set_color_on_pixel(color, xy_to_index(position.0 + (i - 5), position.1 -1), frame);
        set_color_on_pixel(color, xy_to_index(position.0 + (i - 5), position.1), frame);
        set_color_on_pixel(color, xy_to_index(position.0 + (i - 5), position.1 +1), frame);
    }

    // Strich von links nach rechts
    for i in 0u32..11{
        set_color_on_pixel(color, xy_to_index(position.0 -1, position.1 + (i - 5)), frame);
        set_color_on_pixel(color, xy_to_index(position.0, position.1 + (i - 5)), frame);
        set_color_on_pixel(color, xy_to_index(position.0  +1, position.1 + (i - 5)), frame);
    }
}

fn xy_to_index(x: u32, y:u32) -> u32 {
    return y * SPIELFELDGROESSE.0 + x;
}




fn set_border(color: u32, gameframe: &mut Frame) {
    // Decke
    for i in 0..SPIELFELDGROESSE.0 {
        set_color_on_pixel(color, i, gameframe);
        set_color_on_pixel(color, i + SPIELFELDGROESSE.0, gameframe)
    }
    // Boden
    for i in 0..SPIELFELDGROESSE.0 {
        set_color_on_pixel(
            color,
            i + (SPIELFELDGROESSE.0 * (SPIELFELDGROESSE.1 - 1)),
            gameframe,
        );
        set_color_on_pixel(
            color,
            i + (SPIELFELDGROESSE.0 * (SPIELFELDGROESSE.1 - 1)) - SPIELFELDGROESSE.0,
            gameframe,
        );
    }
    // Links
    for i in 0..SPIELFELDGROESSE.1 {
        set_color_on_pixel(color, i * SPIELFELDGROESSE.0, gameframe);
        set_color_on_pixel(color, i * SPIELFELDGROESSE.0 + 1, gameframe);
    }
    // Rechts
    for i in 0..SPIELFELDGROESSE.1 {
        set_color_on_pixel(
            color,
            i * SPIELFELDGROESSE.0 + SPIELFELDGROESSE.0 - 1,
            gameframe,
        );
        set_color_on_pixel(
            color,
            i * SPIELFELDGROESSE.0 + SPIELFELDGROESSE.0 - 2,
            gameframe,
        );
    }
}