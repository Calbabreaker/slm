use std::iter::Peekable;

use crate::{Error, ErrorKind, Literal, Position, Token, TokenKind};

pub struct NodeCall {
    pub identifier: String,
    pub identifier_position: Position,
    pub argument: NodeExpression,
}

pub struct NodeExpression {
    pub literal: Literal,
    pub position: Position,
}

#[derive(Default)]
pub struct NodeRoot {
    pub statements: Vec<NodeCall>,
}

pub struct Parser {
    tokens: Peekable<std::vec::IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().peekable(),
        }
    }

    // call -> identifier(expression)
    // expression -> literal
    pub fn parse(mut self) -> crate::Result<NodeRoot> {
        let mut node_root = NodeRoot::default();

        // Tokens will always end with EOF
        while self.peek_token().kind != TokenKind::EOF {
            let token = self.next_token();
            if let TokenKind::Identifier(identifier) = token.kind {
                node_root.statements.push(NodeCall {
                    identifier,
                    identifier_position: token.position,
                    argument: self.parse_expr()?,
                });
            } else {
                return Err(Error::new(
                    ErrorKind::ExpectedToken("identifer", token.kind),
                    token.position,
                ));
            }
        }

        Ok(node_root)
    }

    fn parse_expr(&mut self) -> Result<NodeExpression, Error> {
        let token = self.next_token();
        if let TokenKind::Literal(literal) = token.kind {
            Ok(NodeExpression {
                literal,
                position: token.position,
            })
        } else {
            Err(Error::new(
                ErrorKind::ExpectedToken("literal", token.kind),
                token.position,
            ))
        }
    }

    fn next_token(&mut self) -> Token {
        self.tokens.next().unwrap()
    }

    fn peek_token(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }
}
