use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use usrlib::kernel::runtime::env_variables;

use crate::kernel::shell::{
    command_parser,
    command_parser::EnvPutStatus::{NotEnoughArguments, NotRightCommand},
};

pub const ENVIRONMENT_COMMAND: &str = "env_put";

pub enum EnvPutStatus {
    NotRightCommand,
    Updated,
    Inserted,
    Deleted,
    NotEnoughArguments,
    Error,
}
pub fn check_and_update_env_command(command: String) -> EnvPutStatus {
    //  Befehl aufspalten für ggf argumente
    let command_array: Vec<String> = command.split(" ").map(str::to_string).collect();

    // Haben wir unseren put befehl?
    if !command_array
        .get(0)
        .unwrap()
        .clone()
        .contains(ENVIRONMENT_COMMAND)
    {
        return NotRightCommand;
    }

    // Ist der Befehl vollständig?
    if command_array.len() < 3 {
        return NotEnoughArguments;
    }

    // Speichern der einzelnen Teile
    let var_name = command_array[1].clone();
    let var_content = command_array[2].clone();

    // Gibt es diese Variable schon?
    if !env_variables::env_contains(var_name.clone().as_str()) {
        // Nein? Es wird inserted
        env_variables::env_insert(var_name.as_str(), var_content.as_str());
        return EnvPutStatus::Inserted;
    }

    // Sie gibt es schon, also update
    env_variables::env_insert(var_name.as_str(), var_content.as_str());
    return EnvPutStatus::Updated;
}

pub fn parse_command(command: String) -> (String, Vec<String>) {
    //  Befehl aufspalten für ggf argumente
    let command_array: Vec<String> = command.split(" ").map(str::to_string).collect();

    // Programm Name rausholen
    let main_command = command_array.get(0).unwrap().clone();

    // Argumente mit "" und Leerzeichen zusammenfassen
    let merged_command_array = merge_quoted_args(command_array);

    // Environmentvariablen ersetzen
    let updated_command_array = merged_command_array
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

    kprintln!("Environment Variable von {} ersetzt zu: ", var);
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


/* = = Funktion zum mergen der Argumente in "Quotes"  = = */
fn merge_quoted_args(args: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    let mut in_quotes = false;
    let mut current = String::new();

    // alle Argumente durchgehen
    for arg in args {

        // Sind wir grade in einem zusammengesetzten Argument?
        if in_quotes {
            // Gelöschtes Leerzeichen hinzufügen
            current.push(' ');
            // Neues Argument an String anhängen
            current.push_str(&arg);

            // Sind wir am ende der Quots
            if arg.ends_with('"') {
                // Zusammensetzung beenden
                in_quotes = false;
                // Quotationmark am Ende löschen
                let merged = current.trim_matches('"').to_string();
                // Als neues zusammengefügtes Argument in Vektor inzufügen
                result.push(merged);
                // String wieder leer machen für die nächsten Quotes
                current.clear();
            }

            continue; // Skipt, da wir arg ja bearbeitet haben
        }

        // Beginnt hier ein zusammengesetztes Wort?
        if arg.starts_with('"') {

            // Wenn es direkt wieder damit Endet, brauchen wir nix zu mergen
            if arg.ends_with('"') && arg.len() > 1 {
                // Abschneiden der Quotes auf beiden Seiten
                let merged = arg.trim_matches('"').to_string();
                // Direkt hinzufügen zu den Argumenten
                result.push(merged);

                continue; // Skipt, da wir arg ja bearbeitet haben
            }
            // Markierung, das wir jetzt ein zusammengesetztes Argument einlesen
            in_quotes = true;
            // Argument in unseren zusammengesetzten String packen
            current.push_str(&arg);

            continue; // Skip pushing to result again
        }

        // Normales Argument einfach in den Vektor schreiben
        result.push(arg);
    }

    // Optional: Fehlerbehandlung, falls ein Anführungszeichen nicht geschlossen wurde
    if in_quotes {
        // Einfach den bisher zusammengesetzten String noch in die Argumente packen
        result.push(current);
    }

    // Neuer Argumenten Vektor
    return result;
}
