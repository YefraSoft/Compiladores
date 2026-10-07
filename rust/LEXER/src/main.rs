use afd::{Afd, check_per_character, create, print_afd, start_state};
use std::env;
use std::io::{self, Read};
use std::process;

mod yeff;

const USAGE: &str = "Uso: cargo run -- <archivo.yeff>  (la cadena se lee por stdin)";

fn main() {
    let Some(arg) = env::args().nth(1) else {
        eprintln!("{USAGE}");
        process::exit(1);
    };

    let path = yeff::path_from_arg(&arg);
    let lines = yeff::read(&path).unwrap_or_else(|error| fail(&error));

    if lines.is_empty() {
        fail(&format!("{path} no contiene transiciones."));
    }

    let afd = create(&lines).unwrap_or_else(|error| fail(&error));
    print_afd(&afd);

    let input = read_stdin();
    let entrada = input.trim();

    println!("\nEntrada leida: {entrada}");

    println!("\nTokens: {}", tokens(&afd, entrada));
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

fn read_stdin() -> String {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .unwrap_or_else(|error| fail(&format!("No se pudo leer stdin: {error}")));

    input
}

fn fail(error: &str) -> ! {
    eprintln!("Error: {error}");
    process::exit(1);
}
