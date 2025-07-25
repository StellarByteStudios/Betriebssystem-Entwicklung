#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;

use usrlib::{
    gprint, gprintln,
    kernel::{
        shell::shell_handler::{activate_shell, deactivate_shell},
        syscall::user_api::usr_getlastkey,
    },
};
use usrlib::kernel::syscall::keyboard::get_last_key;

#[link_section = ".main"]
#[no_mangle]
pub fn main() {
    deactivate_shell();

    gprintln!("Rechner Modus - mit e beendet man den Modus");
    let mut input = String::new();

    loop {
        let key = get_last_key();

        // exit Rechner Modus
        if key == 'e' {
            break;
        }

        // Eingabe eine Formel (Enter)
        if key == '\r' {
            if let Some((left, op, right)) = parse_expression(&input) {
                let result = calculate(left, op, right);
                gprintln!(" = {}", result);
            } else {
                gprintln!("Fehlerhafte Eingabe!");
            }
            input.clear();
        } else {
            input.push(key);
            gprint!("{}", key);
        }
    }

    activate_shell();
}

fn parse_expression(expr: &String) -> Option<(i32, char, i32)> {
    let expr = expr.trim();

    // Liste an Operatoren
    let operators = ['+', '-', '*', '/'];

    // iteriere über chars um Operator zu finden
    for (i, c) in expr.char_indices() {
        if operators.contains(&c) && i != 0 {
            let left_str = &expr[..i].trim();
            let right_str = &expr[i + 1..].trim();

            if let (Ok(left), Ok(right)) = (left_str.parse::<i32>(), right_str.parse::<i32>()) {
                return Some((left, c, right));
            }
        }
    }

    None
}

fn calculate(a: i32, op: char, b: i32) -> i32 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => {
            if b == 0 {
                gprintln!("Fehler: Division durch 0");
                0
            } else {
                a / b
            }
        }
        _ => 0,
    }
}
