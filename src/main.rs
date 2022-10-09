use std::{
    io::{stdout, Write},
    process::exit,
};

use anyhow::Result;
use nix::{libc::STDIN_FILENO, unistd::isatty};
use risp::{eval, print_expr, read};

fn main() -> Result<()> {
    if let Ok(true) = isatty(STDIN_FILENO) {
        read_eval_print_loop()?;
    } else {
        let expr = read()?;
        eval(expr);
    }
    Ok(())
}

fn read_eval_print_loop() -> Result<()> {
    print!("risp> ");
    stdout().flush().unwrap();

    loop {
        match read() {
            Ok(expr) => {
                let expr = eval(expr);
                print_expr(expr);
            }
            Err(ref error) if error.to_string() == "EOF" => {
                exit(1);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        };
        print!("risp> ");
        stdout().flush().unwrap();
    }
}
