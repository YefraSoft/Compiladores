use std::fs;

pub const EXTENSION: &str = ".yeff";

const COMMENT_PREFIXES: [&str; 2] = ["#", "//"];

pub fn path_from_arg(arg: &str) -> String {
    if arg.ends_with(EXTENSION) {
        arg.to_string()
    } else {
        format!("{arg}{EXTENSION}")
    }
}

pub fn read(path: &str) -> Result<Vec<String>, String> {
    let content =
        fs::read_to_string(path).map_err(|error| format!("No se pudo leer {path}: {error}"))?;

    Ok(clean(&content))
}

pub fn clean(content: &str) -> Vec<String> {
    content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter(|line| !is_comment(line))
        .map(String::from)
        .collect()
}

fn is_comment(line: &str) -> bool {
    COMMENT_PREFIXES
        .iter()
        .any(|prefix| line.starts_with(prefix))
}