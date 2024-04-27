use crate::{Source, TokenKind};

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    #[error("Invalid token {0:?}")]
    InvalidToken(String),
    #[error("Expected {0} got token '{1:?}'")]
    ExpectedToken(&'static str, TokenKind),
    #[error("Could not find {0}")]
    NotFound(String),
    #[error("Expected a {0}")]
    UnexpectedType(&'static str),
    #[error("Unmatched {0}")]
    Unmatched(&'static str),
}

#[derive(Debug, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

impl Position {
    pub const ZERO: Self = Self::new(0, 0, 0);

    pub const fn new(line: usize, column: usize, length: usize) -> Self {
        Self {
            line,
            column,
            length,
        }
    }
}

impl Error {
    pub fn new(kind: ErrorKind, position: Position) -> Self {
        Self { kind, position }
    }

    pub fn with_source(&self, source: Source) -> ErrorWithSource {
        ErrorWithSource {
            error: self.clone(),
            source,
        }
    }
}

#[derive(Debug)]
pub struct ErrorWithSource {
    pub error: Error,
    pub source: Source,
}

impl std::error::Error for ErrorWithSource {}

impl std::fmt::Display for ErrorWithSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{0}", self.error.kind)?;

        let position = self.error.position;
        let line_num = (position.line + 1).to_string();
        let column_num = position.column + 1;
        let padding = " ".repeat(line_num.len());

        writeln!(
            f,
            "{}--> {}:{}:{}",
            padding, self.source.path, line_num, column_num
        )?;

        // Print the code snippet
        let mut lines = self.source.code.lines();
        if let Some(line_code) = lines.nth(position.line) {
            writeln!(f, " {} | {}", line_num, line_code)?;

            // Show where the error is in the snippet
            write!(
                f,
                "{}    {}{}",
                padding,
                " ".repeat(position.column),
                "^".repeat(position.length)
            )?;
        }

        Ok(())
    }
}

pub type Result<T> = core::result::Result<T, Error>;
