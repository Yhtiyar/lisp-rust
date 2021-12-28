pub struct Lexer {
    input: String,
    read_position: usize,
    ch: Option<char>,
}
#[derive(Debug, PartialEq)]
enum Token {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Comma,
    Dot,
    Identifier(String),
    Number(f64),
    String(String),
    Bool(bool),
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidCharacter(char),
    InvalidNumber(String),
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            read_position: 0,
            ch: None,
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.len() {
            self.ch = None;
        } else {
            self.ch = Some(self.input.chars().nth(self.read_position).unwrap());
        }
        self.read_position += 1;
        self.ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> Result<String, LexerError> {
        let mut result = String::new();
        while let Some(c) = self.ch {
            if !c.is_whitespace() {
                result.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        Ok(result)
    }

    fn read_number(&mut self) -> Result<f64, LexerError> {
        let mut result = String::new();
        while let Some(c) = self.ch {
            if c.is_numeric() || c == '.' {
                result.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        match result.parse::<f64>() {
            Ok(n) => Ok(n),
            Err(_) => Err(LexerError::InvalidNumber(format!(
                "Error parsing number : {}",
                result,
            ))),
        }
    }

    fn read_string(&mut self) -> String {
        let mut result = String::new();
        self.read_char();
        while let Some(c) = self.ch {
            if c == '"' {
                self.read_char();
                break;
            } else {
                result.push(c);
                self.read_char();
            }
        }
        result
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();
        let ch = match self.ch {
            Some(c) => c,
            None => {
                return Ok(Token::EOF);
            }
        };

        match ch {
            '(' => {
                self.read_char();
                Ok(Token::OpenParen)
            }
            ')' => {
                self.read_char();
                Ok(Token::CloseParen)
            }
            '[' => {
                self.read_char();
                Ok(Token::OpenBracket)
            }
            ']' => {
                self.read_char();
                Ok(Token::CloseBracket)
            }
            ',' => {
                self.read_char();
                Ok(Token::Comma)
            }
            '.' => {
                self.read_char();
                Ok(Token::Dot)
            }
            '-' => {
                self.read_char();
                match self.read_number() {
                    Ok(n) => Ok(Token::Number(-n)),
                    Err(e) => Err(e),
                }
            }
            '"' => {
                let s = self.read_string();
                Ok(Token::String(s))
            }
            _ => {
                if ch.is_numeric() {
                    let n = self.read_number();
                    match n {
                        Ok(n) => Ok(Token::Number(n)),
                        Err(e) => Err(e),
                    }
                } else {
                    let ident = self.read_identifier();
                    match ident {
                        Ok(ident) => {
                            if ident == "true" {
                                Ok(Token::Bool(true))
                            } else if ident == "false" {
                                Ok(Token::Bool(false))
                            } else {
                                Ok(Token::Identifier(ident))
                            }
                        }
                        Err(e) => Err(e),
                    }
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            match tok {
                Ok(tok) => {
                    if tok == Token::EOF {
                        tokens.push(tok);
                        break;
                    }
                    tokens.push(tok)
                }
                Err(e) => return Err(e),
            }
        }
        Ok(tokens)
    }
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_new() {
        let input = String::from("(+ -1.2 2)");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::OpenParen));
        assert_eq!(l.next_token(), Ok(Token::Identifier(String::from("+"))));
        assert_eq!(l.next_token(), Ok(Token::Number(-1.2)));
        assert_eq!(l.next_token(), Ok(Token::Number(2.0)));
        assert_eq!(l.next_token(), Ok(Token::CloseParen));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_string() {
        let input = String::from("\"hello\"");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::String(String::from("hello"))));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_number() {
        let input = String::from("-1.2");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::Number(-1.2)));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_identifier() {
        let input = String::from("hello");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::Identifier(String::from("hello"))));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_identifier_with_number() {
        let input = String::from("hello1");
        let mut l = Lexer::new(input);
        assert_eq!(
            l.next_token(),
            Ok(Token::Identifier(String::from("hello1")))
        );
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_identifier_with_space() {
        let input = String::from("hello 1");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::Identifier(String::from("hello"))));
        assert_eq!(l.next_token(), Ok(Token::Number(1.0)));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_identifier_with_space_and_number() {
        let input = String::from("hello 1");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::Identifier(String::from("hello"))));
        assert_eq!(l.next_token(), Ok(Token::Number(1.0)));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_identifier_with_space_and_number_and_space() {
        let input = String::from("hello 1 2");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::Identifier(String::from("hello"))));
        assert_eq!(l.next_token(), Ok(Token::Number(1.0)));
        assert_eq!(l.next_token(), Ok(Token::Number(2.0)));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_with_paranthesis() {
        let input = String::from("(+ 1 2)");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::OpenParen));
        assert_eq!(l.next_token(), Ok(Token::Identifier(String::from("+"))));
        assert_eq!(l.next_token(), Ok(Token::Number(1.0)));
        assert_eq!(l.next_token(), Ok(Token::Number(2.0)));
        assert_eq!(l.next_token(), Ok(Token::CloseParen));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_with_paranthesis_and_space() {
        let input = String::from("( + 1 2 ) ");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::OpenParen));
        assert_eq!(l.next_token(), Ok(Token::Identifier(String::from("+"))));
        assert_eq!(l.next_token(), Ok(Token::Number(1.0)));
        assert_eq!(l.next_token(), Ok(Token::Number(2.0)));
        assert_eq!(l.next_token(), Ok(Token::CloseParen));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_with_paranthesis_and_space_and_number() {
        let input = String::from("( + 1 2 ) 3");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::OpenParen));
        assert_eq!(l.next_token(), Ok(Token::Identifier(String::from("+"))));
        assert_eq!(l.next_token(), Ok(Token::Number(1.0)));
        assert_eq!(l.next_token(), Ok(Token::Number(2.0)));
        assert_eq!(l.next_token(), Ok(Token::CloseParen));
        assert_eq!(l.next_token(), Ok(Token::Number(3.0)));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_with_brackets() {
        let input = String::from("[1 2]");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Ok(Token::OpenBracket));
        assert_eq!(l.next_token(), Ok(Token::Number(1.0)));
        assert_eq!(l.next_token(), Ok(Token::Number(2.0)));
        assert_eq!(l.next_token(), Ok(Token::CloseBracket));
        assert_eq!(l.next_token(), Ok(Token::EOF));
    }

    #[test]
    fn test_lexer_invalid_number() {
        let input = String::from("1.2.3");
        let mut l = Lexer::new(input);
        assert!(matches!(l.next_token(), Err(LexerError::InvalidNumber(_))));
    }
}
