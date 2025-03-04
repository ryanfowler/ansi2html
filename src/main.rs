use std::io::{Read as _, Write};

use ansi_to_html::convert;

fn main() {
    if let Err(err) = run() {
        let mut stderr = std::io::stderr();
        let _ = stderr.write_all("error: ".as_bytes());
        let _ = stderr.write_all(err.as_bytes());
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|err| err.to_string())?;

    let output = convert(&input).map_err(|err| err.to_string())?;
    let _ = std::io::stdout().write_all(output.as_bytes());

    Ok(())
}
