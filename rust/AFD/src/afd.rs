use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Hash, Eq, PartialEq)]
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

/*
    restructura de trasitions, ya que sobre escribia la misma
*/

pub type Afd = HashMap<State, HashMap<char, State>>;

struct Transition {
    from_name: String,
    event: char,
    to_name: String,
}

pub fn create(lines: &[String]) -> Result<Afd, String> {
    let valid_line =
        Regex::new(r"^[qQ][0-9]+[sf]{0,2} - [a-zA-Z0-9+.*/=();:%!&|,_~-] > [qQ][0-9]+[sf]{0,2}( <[a-zA-Z]+>)?$")
            .unwrap();

    let mut states: HashMap<String, State> = HashMap::new();
    let mut transitions: Vec<Transition> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if !valid_line.is_match(line) {
            return Err(format!("Syntax Error in line: {} value: {}", i, line));
        }

        let sections: Vec<&str> = line.split_whitespace().collect();

        let from_state = build_state(sections[0], None)?;
        let event = sections[2].chars().next().ok_or("Empty transition event")?;
        let to_state = if sections.len() == 6 {
            build_state(sections[4], Some(sections[5]))?
        } else {
            build_state(sections[4], None)?
        };

        let from_name = from_state.name.clone();
        let to_name = to_state.name.clone();

        register_state(&mut states, from_state);
        register_state(&mut states, to_state);
        transitions.push(Transition {
            from_name,
            event,
            to_name,
        });
    }

    check_rules(&states)?;

    let mut afd: Afd = HashMap::new();

    for state in states.values() {
        afd.entry(state.clone()).or_default();
    }

    for transition in transitions {
        let from_state = states
            .get(&transition.from_name)
            .ok_or("Invalid transition state")?
            .clone();
        let to_state = states
            .get(&transition.to_name)
            .ok_or("Invalid transition state")?
            .clone();
        let state_transitions = afd.entry(from_state).or_default();

        if state_transitions.contains_key(&transition.event) {
            return Err("Non deterministic transition.".to_string());
        }

        state_transitions.insert(transition.event, to_state);
    }

    Ok(afd)
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
    afd.keys()
        .find(|state| state.is_start)
        .cloned()
        .ok_or_else(|| "Afd dont have a start state.".to_string())
}

pub fn check(afd: &Afd, line: &str) -> Result<bool, String> {
    let mut state = start_state(afd)?;

    for character in line.chars() {
        let Some(states) = afd.get(&state) else {
            return Ok(false);
        };

        let Some(next_state) = states.get(&character) else {
            return Ok(false);
        };

        state = next_state.clone();
    }

    Ok(state.is_final)
}

pub fn check_per_character(afd: &Afd, state: &State, value: char) -> Result<Option<State>, String> {
    let states = afd.get(state).ok_or("State not found")?;

    Ok(states.get(&value).cloned())
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

    for (state, transitions) in afd {
        // Imprimir información del estado
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

        // Imprimir sus transiciones
        let mut transitions: Vec<(&char, &State)> = transitions.iter().collect();
        transitions.sort_by(|a, b| a.0.cmp(b.0));

        for (event, to_state) in transitions {
            println!("  --{}--> {}", event, to_state.name);
        }

        println!();
    }

    println!("=========================");
}
