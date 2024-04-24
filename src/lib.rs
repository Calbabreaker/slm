mod compiler;
mod error;
mod lexer;
mod parser;

pub use compiler::*;
pub use error::*;
pub use lexer::*;
pub use parser::*;

#[derive(Debug)]
pub struct Source {
    pub code: String,
    pub path: String,
}

impl Source {
    pub fn from_file(path: &std::path::Path) -> std::io::Result<Self> {
        let code = std::fs::read_to_string(path)?;
        Ok(Source {
            code,
            path: path.to_string_lossy().to_string(),
        })
    }
}

/// Try to compile the code returning a result type with the error when there is one
pub fn compile(source: &Source) -> crate::Result<String> {
    let tokens = Lexer::new(&source.code).parse()?;
    let tree = Parser::new(tokens).parse()?;
    let asm = Compiler::default().compile(tree)?;
    Ok(asm)
}
