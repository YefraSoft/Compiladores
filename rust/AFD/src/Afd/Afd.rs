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

pub fn create(lines: &HashSet<String>) -> Result<Afd, String> {
    let valid_line = Regex::new(r"^[qQ][0-9]+s?f? - [a-zA-Z0-9+./-] > [qQ][0-9]+s?f?$").unwrap();

    let mut counter: StatesCounter = HashMap::new();
    let mut lines_counter: i8 = 0;
    let mut afd: Afd = HashMap::new();
    let mut has_s_state = false;
    let mut has_f_state = false;

    for line in lines {
        lines_counter += 1;

        if !valid_line.is_match(line) {
            return Err(format!("Syntax Error in line: {} value: {}", lines_counter, line));
        }

        let sections: Vec<&str> = line.split_whitespace().collect();

        let from_state = build_state(sections[0])?;
        let event = sections[2].chars().next().ok_or("Empty transition event")?;
        let to_state = build_state(sections[4])?;

        if !has_s_state {
            if from_state.is_start && to_state.is_start {
                Err("Invalid transition state".to_string())?;
            } else if from_state.is_start || to_state.is_start {
                has_s_state = true;
            }
        } else if has_f_state {
            if from_state.is_start || to_state.is_start {
                Err("Invalid transition state".to_string())?;
            }
        }

        if !has_f_state {
            if from_state.is_final || to_state.is_final {
                has_f_state = true;
            }
        }

        let from_name = from_state.name.clone();

        // Agrega la transición:
        afd.entry(from_state).or_default().insert(event, to_state);

        // Cuenta cuántas transiciones salen de este estado
        *counter.entry(from_name).or_insert(0) += 1;
    }

    if !has_f_state {
        Err("Don't have a final state".to_string())?;
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
        if c == 's' {
            is_start = true;
        }
        if c == 'f' {
            is_final = true;
        } else {
            name.push(c);
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
