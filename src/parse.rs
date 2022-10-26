use anyhow::{bail, Context, Result};

use crate::expr::Expr;

pub fn parse(src: String) -> Result<Expr> {
    let tokens = Tokenizer::new(src).tokenize()?;
    Parser::new(tokens).parse()
}

struct Tokenizer {
    input: String,
    pos: usize,
}

impl Tokenizer {
    fn new(input: String) -> Self {
        Self { input, pos: 0 }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = vec![];
        while let Some(c) = self.next_char() {
            match c {
                '0'..='9' => {
                    tokens.push(self.read_integer());
                }
                _ => unreachable!(),
            }
        }
        Ok(tokens)
    }

    fn read_integer(&mut self) -> Token {
        let integer = self.consume_while(char::is_ascii_digit);
        Token::Integer(integer)
    }

    fn next_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn consume_while<F>(&mut self, test: F) -> String
    where
        F: Fn(&char) -> bool,
    {
        let mut result = String::new();
        while let Some(c) = self.next_char().and_then(|c| test(&c).then_some(c)) {
            self.pos += 1;
            result.push(c);
        }
        result
    }
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

#[derive(PartialEq, Debug)]
enum Token {
    Integer(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_integer() -> Result<()> {
        let tokens = Tokenizer::new("123".to_string()).tokenize()?;

        assert_eq!(tokens, vec![Token::Integer("123".to_string())]);
        Ok(())
    }

    #[test]
    fn parse_integer() -> Result<()> {
        let result = parse("123".to_string())?;

        assert_eq!(result, Expr::Integer(123));
        Ok(())
    }
}
