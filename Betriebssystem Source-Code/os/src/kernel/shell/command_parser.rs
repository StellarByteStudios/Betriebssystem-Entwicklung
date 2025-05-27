use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use spin::Mutex;

use crate::kernel::shell::{command_parser, env_variables};

pub fn parse_command(command: String) -> (String, Vec<String>) {
    //  Befehl aufspalten für ggf argumente
    let command_array: Vec<String> = command.split(" ").map(str::to_string).collect();

    // Programm Name rausholen
    let main_command = command_array.get(0).unwrap().clone();

    // Environmentvariablen ersetzen
    let updated_command_array = command_array
        .into_iter()
        .map(map_string_to_env_var)
        .collect();

    // Variablen wieder zurückgeben
    return (main_command.to_string(), updated_command_array);
}

pub fn map_string_to_env_var(var: String) -> String {
    // Erstmal Prüfen, ob der String mit einem $ Anfängt
    if !var.starts_with('$') {
        return var;
    }

    kprint!("Environment Variable von {} ersetzt zu: ", var);
    // Variablenname rausschneiden
    let new_var = var.replace('$', "");

    // Variable Suchen
    let env_var = env_variables::env_get(new_var.as_str());

    // War die Suche Erfolgslos
    if env_var.is_none() {
        kprintln!("ERROR");
        return var;
    }

    kprint!("{}", env_var.clone().unwrap());

    // Variablenkontent auspacken und zurückgeben
    return env_var.unwrap();
}
