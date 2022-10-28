use anyhow::{bail, Context, Result};

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Integer(String),
}

pub fn tokenize(input: String) -> Result<Vec<Token>> {
    Tokenizer::new(input).tokenize()
}

#[derive(Debug)]
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
        while let Ok(token) = self
            .read_integer()
            .with_context(|| format!("Unknown token: {:?}", self.next_char()))
        {
            tokens.push(token)
        }
        Ok(tokens)
    }

    fn read_integer(&mut self) -> Result<Token> {
        let c = self.next_char().context("EOF")?;
        match c {
            '0'..='9' => {
                let integer = self.consume_while(char::is_ascii_digit);
                Ok(Token::Integer(integer))
            }
            _ => bail!("Not integer"),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_integer() -> Result<()> {
        let result = Tokenizer::new("123".to_string()).tokenize()?;

        assert_eq!(result, vec![Token::Integer("123".to_string())]);
        Ok(())
    }
}
