use anyhow::{bail, Context, Result};

#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    Number(String),
    Quote,
    Identifier(String),
    WhiteSpace,
    Dot,
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
            .read_number()
            .or_else(|_| self.read_quote())
            .or_else(|_| self.read_identifier())
            .or_else(|_| self.read_whitespace())
            .or_else(|_| self.read_dot())
            .with_context(|| format!("Unknown token: {:?}", self.next_char()))
        {
            tokens.push(token)
        }
        Ok(tokens)
    }

    fn read_number(&mut self) -> Result<Token> {
        let c = self.next_char().context("EOF")?;
        match c {
            '0'..='9' => {
                let number = self.consume_while(char::is_ascii_digit);
                Ok(Token::Number(number))
            }
            _ => bail!("Not number"),
        }
    }

    fn read_quote(&mut self) -> Result<Token> {
        let c = self.next_char().context("EOF")?;
        match c {
            '\'' => {
                self.consume();
                Ok(Token::Quote)
            }
            _ => bail!("Not ascii"),
        }
    }

    fn read_identifier(&mut self) -> Result<Token> {
        let c = self.next_char().context("EOF")?;
        c.is_alphabetic()
            .then(|| self.consume_while(|c| c.is_alphabetic()))
            .map(Token::Identifier)
            .context("Not identifier")
    }

    fn read_whitespace(&mut self) -> Result<Token> {
        let c = self.next_char().context("EOF")?;
        if c == ' ' {
            self.consume();
            Ok(Token::WhiteSpace)
        } else {
            bail!("Not whitespace");
        }
    }

    fn read_dot(&mut self) -> Result<Token> {
        let c = self.next_char().context("EOF")?;
        if c == '.' {
            self.consume();
            Ok(Token::Dot)
        } else {
            bail!("Not dot");
        }
    }

    fn next_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn consume(&mut self) -> Option<char> {
        let result = self.next_char()?;
        self.pos += 1;
        Some(result)
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
    fn tokenize_number() -> Result<()> {
        let result = tokenize("123".to_string())?;

        assert_eq!(result, vec![Token::Number("123".to_string())]);
        Ok(())
    }

    #[test]
    fn tokenize_multiple_number() -> Result<()> {
        let result = tokenize("123 456".to_string())?;

        assert_eq!(
            result,
            vec![
                Token::Number("123".to_string()),
                Token::WhiteSpace,
                Token::Number("456".to_string())
            ]
        );
        Ok(())
    }

    #[test]
    fn tokenize_quote() -> Result<()> {
        let result = tokenize("'atom".to_string())?;

        assert_eq!(
            result,
            vec![Token::Quote, Token::Identifier("atom".to_string())]
        );
        Ok(())
    }

    #[test]
    fn tokenize_float() -> Result<()> {
        let result = tokenize("123.456".to_string())?;

        assert_eq!(
            result,
            vec![
                Token::Number("123".to_string()),
                Token::Dot,
                Token::Number("456".to_string())
            ]
        );
        Ok(())
    }
}
