mod expr;
mod parse;

use std::io;

use anyhow::{bail, Result};
use expr::Expr;
use parse::parse;

pub fn read() -> Result<Expr> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                // EOF given
                bail!("EOF");
            } else {
                parse(buffer.trim().to_string())
            }
        }
        Err(e) => bail!(e),
    }
}

pub fn eval(expr: Expr) -> Expr {
    expr
}

pub fn print_expr(expr: Expr) {
    println!("{}", expr);
}
