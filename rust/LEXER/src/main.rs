use afd::{create, print_afd};
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

    println!("\nEntrada leida: {input}");

    // TODO: escaneo de tokens sobre afd::check_per_character
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
