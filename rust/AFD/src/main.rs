#[path = "Afd/Afd.rs"]
mod afd;

use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(input) = args.get(1) else {
        eprintln!("Uso: cargo run -- <archivo.yeff>");
        process::exit(1);
    };

    let file_path = if input.ends_with(".yeff") {
        input.clone()
    } else {
        format!("{input}.yeff")
    };

    let content = match fs::read_to_string(&file_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("No se pudo leer {file_path}: {e}");
            process::exit(1);
        }
    };
    let lines: Vec<String> = content.lines().map(String::from).collect();

    let afd = match afd::create(&lines) {
        Ok(afd) => afd,
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1);
        }
    };

    afd::print_afd(&afd);
}
