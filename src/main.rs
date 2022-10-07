use std::{
    error::Error,
    io::{self, stdout, Write},
};

use nix::{libc::STDIN_FILENO, unistd::isatty};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();
    if let Ok(true) = isatty(STDIN_FILENO) {
        print!("risp> ");
        stdout().flush().unwrap();
    }

    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    print!("{}", buffer);
    Ok(())
}
