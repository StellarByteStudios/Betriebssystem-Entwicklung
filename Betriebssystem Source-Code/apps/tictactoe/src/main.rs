#![no_std]
#![no_main]

extern crate alloc;

use usrlib::{
    gprintln,
    kernel::{
        shell::shell_handler::{activate_shell, deactivate_shell},
        syscall::{keyboard::get_last_key, user_api::usr_getlastkey},
    },
};

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    deactivate_shell();

    let mut board = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];

    let mut round_count = 0;
    loop {
        // Überprüfe, ob es einen Gewinner gibt
        if let Some(winner) = check_winner(&board) {
            if winner == 'X' {
                gprintln!("Spieler 1 hat gewonnen");
            } else {
                gprintln!("Spieler 2 hat gewonnen");
            }
            break;
        }

        if round_count == 9 {
            gprintln!("Unentschieden");
            break;
        }

        match round_count % 2 {
            0 => gprintln!("Spieler 1 ist am Zug"),
            _ => gprintln!("Spieler 2 ist am Zug"),
        }

        gprintln!(
            " {} | {} | {} \n---+---+---\n {} | {} | {} \n---+---+---\n {} | {} | {}",
            board[0][0],
            board[0][1],
            board[0][2],
            board[1][0],
            board[1][1],
            board[1][2],
            board[2][0],
            board[2][1],
            board[2][2]
        );

        let key = get_last_key();

        // ist die Eingabe eine Zahl?
        if key < '1' || key > '9' {
            gprintln!("ungültige Eingabe");
            continue;
        }

        // Index im Board bestimmen (0 bis 8)
        let pos = (key as u8 - b'1') as usize;
        let row = pos / 3;
        let col = pos % 3;

        board[row][col] = if round_count % 2 == 0 { 'X' } else { 'O' };

        round_count += 1;
    }

    activate_shell();
}

fn check_winner(board: &[[char; 3]; 3]) -> Option<char> {
    // Zeilen prüfen
    for row in 0..3 {
        if board[row][0] == board[row][1] && board[row][1] == board[row][2] {
            return Some(board[row][0]);
        }
    }

    // Spalten prüfen
    for col in 0..3 {
        if board[0][col] == board[1][col] && board[1][col] == board[2][col] {
            return Some(board[0][col]);
        }
    }

    // Diagonale links oben ↘ rechts unten
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return Some(board[0][0]);
    }

    // Diagonale rechts oben ↙ links unten
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return Some(board[0][2]);
    }

    None
}
