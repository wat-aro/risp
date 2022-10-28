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
        let exprs = read()?;
        exprs
            .iter()
            .for_each(|expr| eval(expr).iter().for_each(print_expr));
    }
    Ok(())
}

fn read_eval_print_loop() -> Result<()> {
    print!("risp> ");
    stdout().flush().unwrap();

    loop {
        match read() {
            Ok(exprs) => exprs.iter().for_each(|expr| {
                eval(expr).iter().for_each(print_expr);
            }),
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
