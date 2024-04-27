use std::iter::Peekable;

use crate::{Error, Literal, Position, Token, TokenKind};

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

        // Token list will always end with EOF
        while self.peek_token().kind != TokenKind::EOF {
            let (identifier, position) = self.next_token().expect_identifier()?;
            self.next_token().expect(TokenKind::OpenBracket)?;
            node_root.statements.push(NodeCall {
                identifier,
                identifier_position: position,
                argument: self.parse_expr()?,
            });
            self.next_token().expect(TokenKind::CloseBracket)?;
            self.next_token().expect(TokenKind::SemiColon)?;
        }

        Ok(node_root)
    }

    fn parse_expr(&mut self) -> Result<NodeExpression, Error> {
        let (literal, position) = self.next_token().expect_literal()?;
        Ok(NodeExpression { literal, position })
    }

    fn next_token(&mut self) -> Token {
        self.tokens.next().unwrap()
    }

    fn peek_token(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }
}
