use regex::Regex;
use std::collections::HashMap;

const COMMENT_PREFIXES: [&str; 2] = ["#", "//"];

#[derive(Clone)]
pub struct State {
    name: String,
    is_final: bool,
    is_start: bool,
    label: Option<String>,
}

impl State {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_final(&self) -> bool {
        self.is_final
    }

    pub fn is_start(&self) -> bool {
        self.is_start
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

pub struct Afd {
    states: HashMap<String, State>,
    transitions: HashMap<String, HashMap<char, String>>,
}

pub fn create(lines: &[String]) -> Result<Afd, String> {
    let valid_line = Regex::new(
        r"^[qQ][0-9]+[sf]{0,2} - [a-zA-Z0-9+.*/-] > [qQ][0-9]+[sf]{0,2}( <[a-zA-Z]+>)?$",
    )
    .unwrap();

    let mut afd = Afd {
        states: HashMap::new(),
        transitions: HashMap::new(),
    };

    for (i, line) in lines.iter().enumerate() {
        let line = line.trim();

        if is_ignored(line) {
            continue;
        }

        if !valid_line.is_match(line) {
            return Err(format!("Syntax Error in line: {} value: {}", i + 1, line));
        }

        let sections: Vec<&str> = line.split_whitespace().collect();

        let from_state = build_state(sections[0], None)?;
        let event = sections[2].chars().next().ok_or("Empty transition event")?;
        let to_state = build_state(sections[4], sections.get(5).copied())?;

        let from_name = from_state.name.clone();
        let to_name = to_state.name.clone();

        register_state(&mut afd.states, from_state);
        register_state(&mut afd.states, to_state);

        let state_transitions =
            afd.transitions.entry(from_name).or_default();

        if state_transitions.insert(event, to_name).is_some() {
            return Err(format!("Non deterministic transition in line: {}", i + 1));
        }
    }

    check_rules(&afd.states)?;

    Ok(afd)
}

pub fn is_ignored(line: &str) -> bool {
    let line = line.trim();
    line.is_empty()
        || COMMENT_PREFIXES
            .iter()
            .any(|prefix| line.starts_with(prefix))
}

fn register_state(states: &mut HashMap<String, State>, state: State) {
    states
        .entry(state.name.clone())
        .and_modify(|known_state| {
            known_state.is_final |= state.is_final;
            known_state.is_start |= state.is_start;

            if known_state.label.is_none() {
                known_state.label = state.label.clone();
            }
        })
        .or_insert(state);
}

fn check_rules(states: &HashMap<String, State>) -> Result<(), String> {
    let start_states = states.values().filter(|state| state.is_start).count();

    if start_states == 0 {
        return Err("Afd dont have a start state.".to_string());
    }

    if start_states > 1 {
        return Err("More than one start state.".to_string());
    }

    Ok(())
}

pub fn start_state(afd: &Afd) -> Result<State, String> {
    afd.states
        .values()
        .find(|state| state.is_start)
        .cloned()
        .ok_or_else(|| "Afd dont have a start state.".to_string())
}

pub fn check(afd: &Afd, line: &str) -> Result<bool, String> {
    let mut state = start_state(afd)?;

    for character in line.chars() {
        let Some(next_state) = check_per_character(afd, &state, character)? else {
            return Ok(false);
        };

        state = next_state;
    }

    Ok(state.is_final)
}

pub fn check_per_character(afd: &Afd, state: &State, value: char) -> Result<Option<State>, String> {
    let Some(next_name) = afd
        .transitions
        .get(&state.name)
        .and_then(|transitions| transitions.get(&value))
    else {
        return Ok(None);
    };

    let next_state = afd.states.get(next_name).ok_or("State not found")?;

    Ok(Some(next_state.clone()))
}

fn build_state(state: &str, label: Option<&str>) -> Result<State, String> {
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
        label: label.map(String::from),
    })
}

pub fn print_afd(afd: &Afd) {
    println!("========== AFD ==========");

    let mut states: Vec<&State> = afd.states.values().collect();
    states.sort_by(|a, b| a.name.cmp(&b.name));

    for state in states {
        print!("{}", state.name);

        if state.is_start {
            print!(" [START]");
        }

        if state.is_final {
            print!(" [FINAL]");
        }

        if let Some(label) = &state.label {
            print!(" {label}");
        }

        println!();

        if let Some(transitions) = afd.transitions.get(&state.name) {
            let mut transitions: Vec<(&char, &String)> = transitions.iter().collect();
            transitions.sort_by(|a, b| a.0.cmp(b.0));

            for (event, to_name) in transitions {
                println!("  --{}--> {}", event, to_name);
            }
        }

        println!();
    }

    println!("=========================");
}
