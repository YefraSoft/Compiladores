use regex::Regex;
use std::collections::HashMap;

mod afd {
    use std::collections::{HashMap, HashSet};

    #[derive(Hash, Eq, PartialEq)]
    struct State {
        name: String,
        is: Option<char>,
    }

    struct Transition {
        event: char,
        state: State,
    }

    type Afd = HashMap<State, Transition>;
    type StatesCounter = HashMap<String, i8>;

    // LINE:  1 -a> 2

    pub fn create(&lines: HashSet) -> Result<Afd, String> {
        //                                  {0}         {1}     {2}        {3}      {4}
        const VALID_LINE = Regex::new(r"^[qQ][0-9]+[sf]? - [a-zA-Z0-9+./-] > [qQ][0-9]+[sf]?$").unwrap();
        let mut doc_line = 0;
        let mut counter: StatesCounter = HashMap::new();
        let mut afd: Afd = HashMap::new();

        for line in lines.iter() {
            if VALID_LINE.is_match(line) {
                doc_line += 1;

                let sections: Vec<&str> = line.split_whitespace().collect();
                let from_state = build_state(sections[0])?;
                let to_state = build_state(sections[4])?;
                let event = sections[2];
                let transition = Transition { event, to_state };
                afd.insert(from_state, transition);
                *counter.entry(from_state.name).or_insert(0) += 1;
            } else {
                return Err(format!("Syntax Error in line: {}", line));
            }
        }
    }

    fn build_state(state: &str) -> Result<State, String> {
        let mut name = String::new();
        let mut is = None;
        for c in state.chars() {
            if c == 's' || c == 'f' {
                is = Some(c);
            } else {
                name.push(c);
            }
        }
        Ok(State { name, is })
    }
}

/*
use regex::Regex;
use std::collections::{HashMap, HashSet};

mod afd {
    use regex::Regex;
    use std::collections::{HashMap, HashSet};

    #[derive(Hash, Eq, PartialEq)]
    struct State {
        name: String,
        is: Option<char>,
    }

    type Afd = HashMap<State, HashMap<char, State>>;
    type StatesCounter = HashMap<String, i8>;

    pub fn create(lines: &HashSet<String>) -> Result<Afd, String> {
        let valid_line =
            Regex::new(
                r"^[qQ][0-9]+[sf]? - [a-zA-Z0-9+./-] > [qQ][0-9]+[sf]?$"
            ).unwrap();

        let mut counter: StatesCounter = HashMap::new();
        let mut afd: Afd = HashMap::new();

        for line in lines {
            if !valid_line.is_match(line) {
                return Err(format!("Syntax Error in line: {}", line));
            }

            let sections: Vec<&str> = line.split_whitespace().collect();

            let from_state = build_state(sections[0])?;
            let event = sections[2]
                .chars()
                .next()
                .ok_or("Empty transition event")?;

            let to_state = build_state(sections[4])?;

            let from_name = from_state.name.clone();

            // Agrega la transición:
            //
            // from_state --event--> to_state
            afd.entry(from_state)
                .or_default()
                .insert(event, to_state);

            // Cuenta cuántas transiciones salen de este estado
            *counter.entry(from_name).or_insert(0) += 1;
        }

        Ok(afd)
    }

    fn build_state(state: &str) -> Result<State, String> {
        let mut name = String::new();
        let mut is = None;

        for c in state.chars() {
            if c == 's' || c == 'f' {
                is = Some(c);
            } else {
                name.push(c);
            }
        }

        if name.is_empty() {
            return Err(format!("Invalid state: {}", state));
        }

        Ok(State { name, is })
    }
}
*/
