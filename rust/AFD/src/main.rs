use afd::{check, create};
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let (Some(input), Some(line)) = (args.get(1), args.get(2)) else {
        eprintln!("Uso: cargo run -- <archivo.yeff> <cadena>");
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

    let afd = match create(&lines) {
        Ok(afd) => afd,
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1);
        }
    };

    match check(&afd, line) {
        Ok(true) => println!("Cadena aceptada."),
        Ok(false) => println!("Cadena rechazada."),
        Err(error) => {
            eprintln!("Error: {error}");
            process::exit(1);
        }
    }
}
