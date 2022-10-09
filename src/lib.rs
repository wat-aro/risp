mod value;

use std::{error::Error, io};

use value::{Value, Value::Integer};

pub fn read() -> Result<Value, Box<dyn Error>> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let integer = buffer.trim().parse::<i64>()?;
    Ok(Integer(integer))
}
