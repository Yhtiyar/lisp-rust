pub struct Lexer {
    input: String,
    position: usize,
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
    EOF,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
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
        self.position = self.read_position;
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

    fn read_identifier(&mut self) -> String {
        let mut result = String::new();
        while let Some(c) = self.ch {
            if !c.is_whitespace() {
                result.push(c);
                self.read_char();
            } else {
                break;
            }
        }
        result
    }

    fn read_number(&mut self) -> f64 {
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
            Ok(n) => n,
            Err(_) => panic!("Error parsing number : {}", result),
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

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let ch = match self.ch {
            Some(c) => c,
            None => {
                return Token::EOF;
            }
        };

        match ch {
            '(' => {
                self.read_char();
                Token::OpenParen
            }
            ')' => {
                self.read_char();
                Token::CloseParen
            }
            '[' => {
                self.read_char();
                Token::OpenBracket
            }
            ']' => {
                self.read_char();
                Token::CloseBracket
            }
            ',' => {
                self.read_char();
                Token::Comma
            }
            '.' => {
                self.read_char();
                Token::Dot
            }
            '-' => {
                self.read_char();
                Token::Number(-self.read_number())
            }
            '"' => {
                let s = self.read_string();
                Token::String(s)
            }
            _ => {
                if ch.is_numeric() {
                    let n = self.read_number();
                    Token::Number(n)
                } else {
                    let ident = self.read_identifier();
                    Token::Identifier(ident)
                }
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            if tok == Token::EOF {
                tokens.push(tok);
                break;
            }
            tokens.push(tok);
        }
        tokens
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
        assert_eq!(l.next_token(), Token::OpenParen);
        assert_eq!(l.next_token(), Token::Identifier(String::from("+")));
        assert_eq!(l.next_token(), Token::Number(-1.2));
        assert_eq!(l.next_token(), Token::Number(2.0));
        assert_eq!(l.next_token(), Token::CloseParen);
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_string() {
        let input = String::from("\"hello\"");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::String(String::from("hello")));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_number() {
        let input = String::from("-1.2");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::Number(-1.2));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_identifier() {
        let input = String::from("hello");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::Identifier(String::from("hello")));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_identifier_with_number() {
        let input = String::from("hello1");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::Identifier(String::from("hello1")));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_identifier_with_space() {
        let input = String::from("hello 1");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::Identifier(String::from("hello")));
        assert_eq!(l.next_token(), Token::Number(1.0));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_identifier_with_space_and_number() {
        let input = String::from("hello 1");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::Identifier(String::from("hello")));
        assert_eq!(l.next_token(), Token::Number(1.0));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_identifier_with_space_and_number_and_space() {
        let input = String::from("hello 1 2");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::Identifier(String::from("hello")));
        assert_eq!(l.next_token(), Token::Number(1.0));
        assert_eq!(l.next_token(), Token::Number(2.0));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_with_paranthesis() {
        let input = String::from("(+ 1 2)");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::OpenParen);
        assert_eq!(l.next_token(), Token::Identifier(String::from("+")));
        assert_eq!(l.next_token(), Token::Number(1.0));
        assert_eq!(l.next_token(), Token::Number(2.0));
        assert_eq!(l.next_token(), Token::CloseParen);
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_with_paranthesis_and_space() {
        let input = String::from("( + 1 2 ) ");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::OpenParen);
        assert_eq!(l.next_token(), Token::Identifier(String::from("+")));
        assert_eq!(l.next_token(), Token::Number(1.0));
        assert_eq!(l.next_token(), Token::Number(2.0));
        assert_eq!(l.next_token(), Token::CloseParen);
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_with_paranthesis_and_space_and_number() {
        let input = String::from("( + 1 2 ) 3");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::OpenParen);
        assert_eq!(l.next_token(), Token::Identifier(String::from("+")));
        assert_eq!(l.next_token(), Token::Number(1.0));
        assert_eq!(l.next_token(), Token::Number(2.0));
        assert_eq!(l.next_token(), Token::CloseParen);
        assert_eq!(l.next_token(), Token::Number(3.0));
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    fn test_lexer_with_brackets() {
        let input = String::from("[1 2]");
        let mut l = Lexer::new(input);
        assert_eq!(l.next_token(), Token::OpenBracket);
        assert_eq!(l.next_token(), Token::Number(1.0));
        assert_eq!(l.next_token(), Token::Number(2.0));
        assert_eq!(l.next_token(), Token::CloseBracket);
        assert_eq!(l.next_token(), Token::EOF);
    }

    #[test]
    #[should_panic]
    fn test_lexer_invalid_number() {
        let input = String::from("1.2.3");
        let mut l = Lexer::new(input);
        l.next_token();
    }
}
