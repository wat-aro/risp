use anyhow::{bail, Context, Result};

use crate::{
    expr::Expr,
    tokenize::{tokenize, Token},
};

pub fn parse(src: String) -> Result<Expr> {
    let tokens = tokenize(src)?;
    Parser::new(tokens).parse()
}

struct Parser {
    input: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(input: Vec<Token>) -> Self {
        Self { input, pos: 0 }
    }

    fn parse(&mut self) -> Result<Expr> {
        self.parse_integer()
    }

    fn parse_integer(&mut self) -> Result<Expr> {
        match self.input.get(self.pos) {
            Some(token) => match token {
                Token::Integer(int) => int
                    .parse::<i64>()
                    .context("Failed parse integer")
                    .map(Expr::Integer),
            },
            None => bail!("EOF"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer() -> Result<()> {
        let result = parse("123".to_string())?;

        assert_eq!(result, Expr::Integer(123));
        Ok(())
    }
}
