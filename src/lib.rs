mod expr;

use std::{error::Error, io};

use expr::{Expr, Expr::Integer};

pub fn read() -> Result<Expr, Box<dyn Error>> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let integer = buffer.trim().parse::<i64>()?;
    Ok(Integer(integer))
}
