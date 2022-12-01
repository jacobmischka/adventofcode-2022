use std::io::{self, Read, Result};

pub fn get_input() -> Result<String> {
    let mut s = String::new();
    let stdin = io::stdin();
    stdin.lock().read_to_string(&mut s)?;
    Ok(s)
}
