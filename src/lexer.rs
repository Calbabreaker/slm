use std::{iter::Peekable, str::Chars};

use crate::{Error, ErrorKind, NodeIdentifier, NodeLiteral, Position};

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Literal {
    Integer(u64),
    String(String),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TokenKind {
    Ident(String),
    Let,
    Equal,
    Return,
    SemiColon,
    Literal(Literal),
    OpenBracket,
    CloseBracket,
    Ignore,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub position: Position,
}

impl Token {
    pub fn expect(&self, kind: TokenKind) -> crate::Result<()> {
        match self.kind == kind {
            true => Ok(()),
            false => Err(Error::new(
                ErrorKind::ExpectedToken(format!("{:?}", kind), self.kind.clone()),
                self.position,
            )),
        }
    }

    pub fn expect_identifier(self) -> crate::Result<NodeIdentifier> {
        match self.kind {
            TokenKind::Ident(value) => Ok(NodeIdentifier(value, self.position)),
            _ => Err(self.make_expect_error("Identifier"))?,
        }
    }

    pub fn expect_literal(self) -> crate::Result<NodeLiteral> {
        match self.kind {
            TokenKind::Literal(value) => Ok(NodeLiteral(value, self.position)),
            _ => Err(self.make_expect_error("Literal"))?,
        }
    }

    pub fn make_expect_error(&self, message: impl Into<String>) -> Error {
        Error::new(
            ErrorKind::ExpectedToken(message.into(), self.kind.clone()),
            self.position,
        )
    }
}

impl Token {
    pub const fn new(kind: TokenKind, position: Position) -> Self {
        Self { kind, position }
    }
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    chars_index: usize,
    code: &'a str,
    current_position: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            chars_index: 0,
            chars: code.chars().peekable(),
            current_position: Position::default(),
        }
    }

    pub fn parse(mut self) -> crate::Result<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            match self.next_token() {
                Err(kind) => Err(Error::new(kind, self.current_position))?,
                Ok(TokenKind::Ignore) => (),
                Ok(TokenKind::EOF) => {
                    // Get the last token position when EOF or it will show nothing
                    let position = tokens.last().map(|p| p.position).unwrap_or_default();
                    tokens.push(Token::new(TokenKind::EOF, position));
                    break;
                }
                Ok(token) => {
                    tokens.push(Token::new(token, self.current_position));
                    self.current_position.column += self.current_position.length;
                }
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<TokenKind, ErrorKind> {
        self.current_position.length = 0;

        Ok(match self.next_char() {
            '\0' => TokenKind::EOF,
            ';' => TokenKind::SemiColon,
            '(' => TokenKind::OpenBracket,
            ')' => TokenKind::CloseBracket,
            '=' => TokenKind::Equal,
            '#' => {
                loop {
                    // Keep consuming until reach a new line to ignore tokens
                    let char = self.peek_char();
                    if char == '\n' || char == '\0' {
                        break;
                    }
                    self.next_char();
                }
                TokenKind::Ignore
            }
            '\n' => {
                self.current_position.line += 1;
                self.current_position.column = 0;
                TokenKind::Ignore
            }
            '"' => {
                loop {
                    let char = self.peek_char();
                    if char == '\n' || char == '\0' {
                        return Err(ErrorKind::Unmatched("\""));
                    };
                    if char == '"' {
                        break;
                    }
                    self.next_char();
                }

                let substr = self.substr().to_string();
                self.next_char();
                TokenKind::Literal(Literal::String(substr))
            }
            char if char.is_alphabetic() || char == '_' => {
                // Keep consuming until not a valid namer
                while self.peek_char().is_alphanumeric() || self.peek_char() == '_' {
                    self.next_char();
                }

                match self.substr() {
                    "return" => TokenKind::Return,
                    "let" => TokenKind::Let,
                    substr => TokenKind::Ident(substr.to_owned()),
                }
            }
            char if char.is_ascii_digit() => {
                while self.peek_char().is_ascii_digit() {
                    self.next_char();
                }

                TokenKind::Literal(Literal::Integer(self.substr().parse().unwrap()))
            }
            char if char.is_whitespace() => {
                self.current_position.column += 1;
                TokenKind::Ignore
            }
            _ => Err(ErrorKind::InvalidToken(self.substr().to_string()))?,
        })
    }

    fn next_char(&mut self) -> char {
        self.current_position.length += 1; // aka. the current token length
        self.chars_index += 1;
        self.chars.next().unwrap_or('\0')
    }

    fn peek_char(&mut self) -> char {
        *self.chars.peek().unwrap_or(&'\0')
    }

    // Get the token substr using the current position
    fn substr(&self) -> &'a str {
        let start_index = self.chars_index - self.current_position.length;
        &self.code[(start_index)..(self.chars_index)]
    }
}
