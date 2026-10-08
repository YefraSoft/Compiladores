use std::fs;

pub const EXTENSION: &str = ".yeff";

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

    Ok(content.lines().map(String::from).collect())
}
