use anyhow::{anyhow, bail, Result};

use crate::{
    expr::Expr,
    tokenize::{tokenize, Token},
};

pub fn parse(src: String) -> Result<Vec<Expr>> {
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

    fn parse(&mut self) -> Result<Vec<Expr>> {
        let mut expressions = Vec::new();
        loop {
            if self.eof() {
                break;
            } else {
                let expr = self.parse_expression().map_err(|e| anyhow!(e))?;
                expressions.push(expr);
            }
        }
        Ok(expressions)
    }

    fn eof(&self) -> bool {
        self.input.len() <= self.pos
    }

    fn parse_expression(&mut self) -> Result<Expr> {
        match self.input.get(self.pos) {
            Some(token) => match token {
                Token::Integer(int) => {
                    self.pos += 1;
                    let integer: i64 = int.chars().fold(0i64, |acc, c| {
                        acc * 10 + (c.to_digit(10u32).unwrap() as i64)
                    });
                    Ok(Expr::Integer(integer))
                }
                Token::Quote => match self.next_token() {
                    Some(token) => match token {
                        Token::Identifier(identifier) => {
                            let atom = Expr::Atom(identifier.clone());
                            self.pos += 2;
                            Ok(atom)
                        }
                        _ => bail!("Not atom"),
                    },
                    None => bail!("Unterminated quote"),
                },
                _ => bail!("Not integer"),
            },
            None => bail!("EOF"),
        }
    }

    fn next_token(&self) -> Option<&Token> {
        self.input.get(self.pos + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_integer() -> Result<()> {
        let result = parse("123".to_string())?;

        assert_eq!(result, vec![Expr::Integer(123)]);
        Ok(())
    }

    #[test]
    fn parse_atom() -> Result<()> {
        let result = parse("'atom".to_string())?;

        assert_eq!(result, vec![Expr::Atom("atom".to_string())]);
        Ok(())
    }
}
