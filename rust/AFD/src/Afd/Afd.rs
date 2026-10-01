use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Hash, Eq, PartialEq)]
pub struct State {
    name: String,
    is_final: bool,
    is_start: bool,
}

/*
    restructura de trasitions, ya que sobre escribia la misma
*/

pub type Afd = HashMap<State, HashMap<char, State>>;
type StatesCounter = HashMap<String, i8>;

pub fn create(lines: &[String]) -> Result<Afd, String> {
    let valid_line =
        Regex::new(r"^[qQ][0-9]+[sf]{0,2} - [a-zA-Z0-9+./-] > [qQ][0-9]+[sf]{0,2}$").unwrap();

    let mut counter: StatesCounter = HashMap::new();
    let mut afd: Afd = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        if !valid_line.is_match(line) {
            return Err(format!("Syntax Error in line: {} value: {}", i, line));
        }

        let sections: Vec<&str> = line.split_whitespace().collect();

        let from_state = build_state(sections[0])?;
        let event = sections[2].chars().next().ok_or("Empty transition event")?;
        let to_state = build_state(sections[4])?;
        let from_name = from_state.name.clone();

        afd.entry(from_state).or_default().insert(event, to_state);
        *counter.entry(from_name).or_insert(0) += 1;
    }

    if afd.keys().filter(|state| state.is_start).count() > 1 {
        return Err("More than one start state.".to_string());
    }

    Ok(afd)
}

pub fn check(afd: &Afd, line: &String) -> Result<bool, String> {
    let mut state = afd
        .keys()
        .find(|state| state.is_start)
        .ok_or_else(|| "Afd dont have a final state".to_string())?;

    for character in line.chars() {
        match afd.get(state) {
            Some(states) => {
                if states.contains_key(&character) {
                    state = &states[&character];
                }
            }
            None => {
                return Err(format!("Unknown character {}", character));
            }
        }
    }
    if state.is_final { Ok(true) } else { Ok(false) }
}

fn build_state(state: &str) -> Result<State, String> {
    let mut name = String::new();
    let mut is_final = false;
    let mut is_start = false;

    for c in state.chars() {
        match c {
            's' => is_start = true,
            'f' => is_final = true,
            _ => name.push(c),
        }
    }

    if name.is_empty() {
        return Err(format!("Invalid state: {}", state));
    }

    Ok(State {
        name,
        is_final,
        is_start,
    })
}

pub fn print_afd(afd: &Afd) {
    println!("========== AFD ==========");

    for (state, transitions) in afd {
        // Imprimir información del estado
        print!("{}", state.name);

        if state.is_start {
            print!(" [START]");
        }

        if state.is_final {
            print!(" [FINAL]");
        }

        println!();

        // Imprimir sus transiciones
        for (event, to_state) in transitions {
            println!("  --{}--> {}", event, to_state.name);
        }

        println!();
    }

    println!("=========================");
}
