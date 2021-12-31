use super::lexer::*;
use super::nodes::*;

use std::error;
use std::fmt;
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    UnexpectedToken(Token, String),
    UnexpectedEndOfFile,
    ParserStateError(String),
}

impl error::Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken(t, s) => write!(f, "Unexpected token: {:?} {}", t, s),
            ParserError::UnexpectedEndOfFile => write!(f, "Unexpected end of file"),
            ParserError::ParserStateError(s) => write!(f, "Parser state error: {}", s),
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            pos: 0,
        }
    }

    pub fn from_source(source: String) -> Result<Parser, LexerError> {
        let tokens = Lexer::new(source).tokenize()?;
        Ok(Parser::new(tokens))
    }

    fn curr_token(&self) -> &Token {
        &self.tokens[self.pos]
    }

    pub fn parse(&mut self) -> Result<Node, ParserError> {
        return self.parseProgram();
    }

    pub fn parseProgram(&mut self) -> Result<Node, ParserError> {
        let mut nodes = vec![];
        while self.pos < self.tokens.len() {
            nodes.push(self.parse_node()?);
        }
        match nodes.last() {
            Some(node) => match node {
                Node::EOF => Ok(Node::Program(nodes)),
                _ => Err(ParserError::ParserStateError("Expected EOF".to_string())),
            },
            None => Err(ParserError::ParserStateError("Empty program".to_string())),
        }
    }

    pub fn parse_list(&mut self) -> Result<Node, ParserError> {
        let mut nodes = vec![];
        while self.curr_token() != &Token::CloseBracket {
            let node = self.parse_node()?;
            if node == Node::EOF {
                return Err(ParserError::UnexpectedEndOfFile);
            }
            nodes.push(node);
        }
        self.pos += 1;
        Ok(Node::Atom(Value::List(nodes)))
    }

    pub fn parse_function_call(&mut self) -> Result<Node, ParserError> {
        let name_node = self.parse_node()?;

        let name = match name_node {
            Node::Variable(name) => name,
            _ => {
                return Err(ParserError::UnexpectedToken(
                    self.curr_token().clone(),
                    format!("{:?} is not a variable", self.curr_token()),
                ))
            }
        };

        let mut args = vec![];
        while self.curr_token() != &Token::CloseParen {
            let node = self.parse_node()?;
            if node == Node::EOF {
                return Err(ParserError::UnexpectedEndOfFile);
            }
            args.push(node);
        }
        self.pos += 1;

        Ok(Node::FunctionCall(name.clone(), args))
    }

    pub fn parse_node(&mut self) -> Result<Node, ParserError> {
        match &self.tokens[self.pos] {
            Token::EOF => {
                self.pos += 1;
                Ok(Node::EOF)
            }
            Token::Number(n) => {
                self.pos += 1;
                Ok(Node::Atom(Value::Number(*n)))
            }
            Token::String(s) => {
                self.pos += 1;
                Ok(Node::Atom(Value::String(s.to_string())))
            }
            Token::Bool(b) => {
                self.pos += 1;
                Ok(Node::Atom(Value::Boolean(*b)))
            }
            Token::Identifier(s) => {
                self.pos += 1;
                Ok(Node::Variable(s.to_string()))
            }
            Token::OpenBracket => {
                self.pos += 1;
                self.parse_list()
            }
            Token::OpenParen => {
                self.pos += 1;
                self.parse_function_call()
            }
            _ => {
                unimplemented!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let tokens = vec![Token::String("hello".to_string()), Token::EOF];
        let mut parser = Parser::new(tokens);
        let node = parser.parse_node().unwrap();
        assert_eq!(node, Node::Atom(Value::String("hello".to_string())));
    }

    #[test]
    fn test_parse_number() {
        let tokens = vec![Token::Number(5.0), Token::EOF];
        let mut parser = Parser::new(tokens);
        let node = parser.parse_node().unwrap();
        assert_eq!(node, Node::Atom(Value::Number(5.0)));
    }

    #[test]
    fn test_parse_bool() {
        let tokens = vec![Token::Bool(true), Token::EOF];
        let mut parser = Parser::new(tokens);
        let node = parser.parse_node().unwrap();
        assert_eq!(node, Node::Atom(Value::Boolean(true)));
    }

    #[test]
    fn test_parse_identifier() {
        let tokens = vec![Token::Identifier("hello".to_string()), Token::EOF];
        let mut parser = Parser::new(tokens);
        let node = parser.parse_node().unwrap();
        assert_eq!(node, Node::Variable("hello".to_string()));
    }

    #[test]
    fn test_parse_program() {
        let tokens = vec![
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();
        assert_eq!(
            program,
            Node::Program(vec![
                Node::Atom(Value::Number(1.0)),
                Node::Atom(Value::Number(2.0)),
                Node::Atom(Value::Number(3.0)),
                Node::EOF,
            ])
        );
    }

    #[test]
    fn test_parse_list() {
        let tokens = vec![
            Token::OpenBracket,
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::CloseBracket,
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let list = parser.parse_node().unwrap();
        assert_eq!(
            list,
            Node::Atom(Value::List(vec![
                Node::Atom(Value::Number(1.0)),
                Node::Atom(Value::Number(2.0)),
                Node::Atom(Value::Number(3.0)),
            ]))
        );
    }

    #[test]
    fn test_parse_nested_list() {
        let tokens = vec![
            Token::OpenBracket,
            Token::String("foo".to_string()),
            Token::OpenBracket,
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::CloseBracket,
            Token::CloseBracket,
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let list = parser.parse_node().unwrap();
        assert_eq!(
            list,
            Node::Atom(Value::List(vec![
                Node::Atom(Value::String("foo".to_string())),
                Node::Atom(Value::List(vec![
                    Node::Atom(Value::Number(1.0)),
                    Node::Atom(Value::Number(2.0)),
                    Node::Atom(Value::Number(3.0)),
                ])),
            ]))
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_not_closed_list() {
        let tokens = vec![
            Token::OpenBracket,
            Token::Number(1.0),
            Token::Number(2.0),
            Token::Number(3.0),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let _list = parser.parse_node().unwrap();
    }

    #[test]
    fn test_parse_function_call() {
        let tokens = vec![
            Token::OpenParen,
            Token::Identifier("foo".to_string()),
            Token::Number(1.0),
            Token::Number(2.0),
            Token::CloseParen,
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let function_call = parser.parse_node().unwrap();
        assert_eq!(
            function_call,
            Node::FunctionCall(
                "foo".to_string(),
                vec![
                    Node::Atom(Value::Number(1.0)),
                    Node::Atom(Value::Number(2.0))
                ]
            )
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_not_closed_function_call() {
        let tokens = vec![
            Token::OpenParen,
            Token::Identifier("foo".to_string()),
            Token::Number(1.0),
            Token::Number(2.0),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let _function_call = parser.parse_node().unwrap();
    }

    #[test]
    fn test_parser_from_source() {
        let source = "
            (defn foo [x y] (+ x y))
            (foo 1 2)
        ";
        let mut parser = Parser::from_source(source.to_owned()).unwrap();
        let program = parser.parse().unwrap();
        assert_eq!(
            program,
            Node::Program(vec![
                Node::FunctionCall(
                    "defn".to_string(),
                    vec![
                        Node::Variable("foo".to_string()),
                        Node::Atom(Value::List(vec![
                            Node::Variable("x".to_string()),
                            Node::Variable("y".to_string()),
                        ])),
                        Node::FunctionCall(
                            "+".to_string(),
                            vec![
                                Node::Variable("x".to_string()),
                                Node::Variable("y".to_string()),
                            ]
                        )
                    ]
                ),
                Node::FunctionCall(
                    "foo".to_string(),
                    vec![
                        Node::Atom(Value::Number(1.0)),
                        Node::Atom(Value::Number(2.0)),
                    ]
                ),
                Node::EOF
            ])
        );
    }
}
