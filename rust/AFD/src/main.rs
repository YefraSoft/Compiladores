#[path = "Afd/Afd.rs"]
mod afd;

use std::collections::HashSet;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = format!("{}.yeff", args[1]);

    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let lines: HashSet<String> = content.lines().map(String::from).collect();

    let afd = match afd::create(&lines) {
        Ok(afd) => afd,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    afd::print_afd(&afd);
}
