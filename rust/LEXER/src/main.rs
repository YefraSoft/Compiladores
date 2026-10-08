use afd::{Afd, check_per_character, create, is_ignored, print_afd, start_state};
use std::env;
use std::io::{self, BufRead, Write};
use std::process;

mod yeff;

const USAGE: &str = "Uso: cargo run -- <archivo.yeff>";

fn main() {
    let Some(arg) = env::args().nth(1) else {
        eprintln!("{USAGE}");
        process::exit(1);
    };

    let path = yeff::path_from_arg(&arg);
    let lines = yeff::read(&path).unwrap_or_else(|error| fail(&error));

    if lines.iter().all(|line| is_ignored(line)) {
        fail(&format!("{path} no contiene transiciones."));
    }

    let afd = create(&lines).unwrap_or_else(|error| fail(&error));
    print_afd(&afd);

    println!("\nIngresa cadenas; para cerrar presiona Control + D");
    read_inputs(&afd);
}

fn read_inputs(afd: &Afd) {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    loop {
        print!("-> ");
        io::stdout()
            .flush()
            .unwrap_or_else(|error| fail(&format!("No se pudo mostrar el indicador: {error}")));

        let Some(line) = lines.next() else {
            println!();
            break;
        };
        let entrada = line.unwrap_or_else(|error| fail(&format!("No se pudo leer stdin: {error}")));

        println!("\nEntrada leida: {entrada}");
        println!("Tokens: {}\n", tokens(afd, &entrada));
    }
}

fn tokens(afd: &Afd, entrada: &str) -> String {
    let characters: Vec<char> = entrada.chars().collect();

    let mut state = start_state(afd).unwrap();
    let mut token = String::new();
    let mut result: Vec<String> = Vec::new();
    let mut i = 0;

    while i < characters.len() {
        let c = characters[i];

        let next = check_per_character(afd, &state, c).unwrap_or_else(|error| fail(&error));

        match next {
            Some(nuevo) => {
                state = nuevo;
                token.push(c);
                i += 1;
            }
            None => {
                if let Some(label) = state.label().map(str::to_string) {
                    push_token(&mut result, &label, &token);
                } else if token.is_empty() {
                    if !c.is_whitespace() {
                        push_token(&mut result, "error", &c.to_string());
                    }
                    i += 1;
                } else {
                    if !c.is_whitespace() {
                        token.push(c);
                    }
                    push_token(&mut result, "error", &token);
                    i += 1;
                }
                token.clear();
                state = start_state(afd).unwrap();
            }
        }
    }

    if !token.is_empty() {
        let label = state.label().unwrap_or("error").to_string();
        push_token(&mut result, &label, &token);
    }

    result.join(", ")
}

fn push_token(result: &mut Vec<String>, label: &str, valor: &str) {
    if valor.chars().all(char::is_whitespace) {
        return;
    }

    result.push(format!("<{label}, {valor}>"));
}

fn fail(error: &str) -> ! {
    eprintln!("Error: {error}");
    process::exit(1);
}
