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

pub enum NodeExpr {
    Literal(NodeLiteral),
    Identifer(NodeIdentifier),
}

pub struct NodeStmtLet {
    pub identifier: NodeIdentifier,
    pub expression: NodeExpr,
}

pub struct NodeStmtCall {
    pub identifier: NodeIdentifier,
    pub argument: NodeExpr,
}

pub enum NodeStmt {
    Let(NodeStmtLet),
    Call(NodeStmtCall),
}

#[derive(Default)]
pub struct NodeProgram {
    pub statements: Vec<NodeStmt>,
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

    // program -> stmt*
    // stmt -> { call, let identifier = expr; }
    // call -> identifier(expression)
    // expr -> { literal, identifier }
    pub fn parse(mut self) -> crate::Result<NodeProgram> {
        let mut node_root = NodeProgram::default();

        // Token list will always end with EOF
        while self.peek_token().kind != TokenKind::EOF {
            node_root.statements.push(self.parse_stmt()?);
        }

        Ok(node_root)
    }

    fn parse_stmt(&mut self) -> crate::Result<NodeStmt> {
        let token = self.next_token();
        let stmt = match token.kind {
            TokenKind::Identifier(identifier) => {
                self.next_token().expect(TokenKind::OpenBracket)?;
                let call = NodeStmtCall {
                    identifier: NodeIdentifier(identifier, token.position),
                    argument: self.parse_expr()?,
                };
                self.next_token().expect(TokenKind::CloseBracket)?;
                NodeStmt::Call(call)
            }
            TokenKind::Let => {
                let identifier = self.next_token().expect_identifier()?;

                self.next_token().expect(TokenKind::Equal)?;
                let stmt = NodeStmtLet {
                    identifier,
                    expression: self.parse_expr()?,
                };

                NodeStmt::Let(stmt)
            }
            _ => Err(token.make_expect_error("Identifier or Let"))?,
        };

        self.next_token().expect(TokenKind::SemiColon)?;
        Ok(stmt)
    }

    fn parse_expr(&mut self) -> crate::Result<NodeExpr> {
        let token = self.next_token();
        match token.kind {
            TokenKind::Literal(literal) => {
                Ok(NodeExpr::Literal(NodeLiteral(literal, token.position)))
            }
            TokenKind::Identifier(identifier) => Ok(NodeExpr::Identifer(NodeIdentifier(
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
