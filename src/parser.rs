use std::iter::Peekable;

use crate::{Error, ErrorKind, Literal, Position, Token, TokenKind};

pub struct NodeLiteral(pub Literal, pub Position);

pub struct NodeIdentifier(pub String, pub Position);

impl NodeIdentifier {
    pub fn make_not_found_error(&self) -> Error {
        Error::new(ErrorKind::NotFound(self.0.clone()), self.1)
    }

    pub fn make_already_used_error(&self) -> Error {
        Error::new(ErrorKind::AlreadyUsed(self.0.clone()), self.1)
    }
}

pub struct NodeAdd {
    pub left: NodeExpression,
    pub right: NodeExpression,
}

pub struct NodeMult {
    pub left: NodeExpression,
    pub right: NodeExpression,
}

pub enum NodeExpression {
    Literal(NodeLiteral),
    Identifer(NodeIdentifier),
}

pub struct NodeLet {
    pub identifier: NodeIdentifier,
    pub expression: NodeExpression,
}

pub struct NodeCall {
    pub identifier: NodeIdentifier,
    pub argument: NodeExpression,
}

pub enum NodeStatement {
    Let(NodeLet),
    Call(NodeCall),
}

#[derive(Default)]
pub struct NodeProgram {
    pub statements: Vec<NodeStatement>,
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

    // program -> statement*
    // statement -> { call, let identifier = expression; }
    // call -> identifier(expression)
    // expression -> { literal, identifier }
    pub fn parse(mut self) -> crate::Result<NodeProgram> {
        let mut node_root = NodeProgram::default();

        // Token list will always end with EOF
        while self.peek_token().kind != TokenKind::EOF {
            node_root.statements.push(self.parse_statement()?);
        }

        Ok(node_root)
    }

    fn parse_statement(&mut self) -> crate::Result<NodeStatement> {
        let token = self.next_token();
        let stmt = match token.kind {
            TokenKind::Ident(identifier) => {
                self.next_token().expect(TokenKind::OpenBracket)?;
                let call = NodeCall {
                    identifier: NodeIdentifier(identifier, token.position),
                    argument: self.parse_expression()?,
                };
                self.next_token().expect(TokenKind::CloseBracket)?;
                NodeStatement::Call(call)
            }
            TokenKind::Let => {
                let identifier = self.next_token().expect_identifier()?;

                self.next_token().expect(TokenKind::Equal)?;
                let stmt = NodeLet {
                    identifier,
                    expression: self.parse_expression()?,
                };

                NodeStatement::Let(stmt)
            }
            _ => Err(token.make_expect_error("Identifier or Let"))?,
        };

        self.next_token().expect(TokenKind::SemiColon)?;
        Ok(stmt)
    }

    fn parse_expression(&mut self) -> crate::Result<NodeExpression> {
        let token = self.next_token();
        match token.kind {
            TokenKind::Literal(literal) => Ok(NodeExpression::Literal(NodeLiteral(
                literal,
                token.position,
            ))),
            TokenKind::Ident(identifier) => Ok(NodeExpression::Identifer(NodeIdentifier(
                identifier,
                token.position,
            ))),
            _ => Err(token.make_expect_error("Literal or Identifier")),
        }
    }

    fn next_token(&mut self) -> Token {
        self.tokens.next().unwrap()
    }

    fn peek_token(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }
}
