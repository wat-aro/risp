use std::{
    error::Error,
    io::{stdout, Write},
};

use nix::{libc::STDIN_FILENO, unistd::isatty};
use risp::read;

fn main() -> Result<(), Box<dyn Error>> {
    if let Ok(true) = isatty(STDIN_FILENO) {
        print!("risp> ");
        stdout().flush().unwrap();
    }

    let result = read()?;
    println!("{}", result);
    Ok(())
}
