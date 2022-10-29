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
        match self.consume() {
            Some(token) => match token {
                Token::Number(int) => {
                    let integer: i64 = int.chars().fold(0i64, |acc, c| {
                        acc * 10 + (c.to_digit(10u32).unwrap() as i64)
                    });

                    if matches!(self.next_token(), Some(Token::Dot)) {
                        self.consume();
                        if let Some(Token::Number(num)) =
                            self.consume_if(|token| matches!(token, Token::Number(_)))
                        {
                            let fractional_part =
                                (1i64..).zip(num.chars()).fold(0f64, |acc, (index, c)| {
                                    acc + c.to_digit(10).unwrap() as f64
                                        * 10f64.powf(-1.0 * index as f64)
                                });
                            Ok(Expr::Float(integer as f64 + fractional_part))
                        } else {
                            Ok(Expr::Float(integer as f64))
                        }
                    } else {
                        Ok(Expr::Integer(integer))
                    }
                }
                Token::Quote => match self.consume() {
                    Some(token) => match token {
                        Token::Identifier(identifier) => {
                            let atom = Expr::Atom(identifier.clone());
                            Ok(atom)
                        }
                        _ => bail!("Not atom"),
                    },
                    None => bail!("Unterminated quote"),
                },
                Token::WhiteSpace => self.parse_expression(),
                _ => Err(anyhow!("Unknown token: {:?}", token)),
            },
            None => bail!("EOF"),
        }
    }

    fn next_token(&self) -> Option<&Token> {
        self.input.get(self.pos)
    }

    fn consume(&mut self) -> Option<&Token> {
        let next_token = self.input.get(self.pos);
        self.pos += 1;
        next_token
    }

    fn consume_if<F>(&mut self, test: F) -> Option<&Token>
    where
        F: Fn(&Token) -> bool,
    {
        let next_token = self.input.get(self.pos)?;
        if test(next_token) {
            self.pos += 1;
            Some(next_token)
        } else {
            None
        }
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
    fn parse_multiple_integer() -> Result<()> {
        let result = parse("123 456".to_string())?;

        assert_eq!(result, vec![Expr::Integer(123), Expr::Integer(456)]);
        Ok(())
    }

    #[test]
    fn parse_atom() -> Result<()> {
        let result = parse("'atom".to_string())?;

        assert_eq!(result, vec![Expr::Atom("atom".to_string())]);
        Ok(())
    }

    #[test]
    fn parse_float() -> Result<()> {
        let result = parse("123.456".to_string())?;

        assert_eq!(result, vec![Expr::Float(123.456)]);
        Ok(())
    }

    #[test]
    fn parse_lack_float() -> Result<()> {
        let result = parse("123. 3".to_string())?;

        assert_eq!(result, vec![Expr::Float(123.0), Expr::Integer(3)]);
        Ok(())
    }
}
