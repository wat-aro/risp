mod expr;

use std::io;

use anyhow::{bail, Result};
use expr::{Expr, Expr::Integer};

pub fn read() -> Result<Expr> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    match stdin.read_line(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                // EOF given
                bail!("EOF");
            } else {
                let integer = buffer.trim().parse::<i64>()?;
                Ok(Integer(integer))
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
