use std::fmt::Write;

use crate::{Error, ErrorKind, Literal, NodeCall, NodeRoot};

#[derive(Default)]
pub struct Compiler {
    out: String,
}

impl Compiler {
    pub fn compile(mut self, root: NodeRoot) -> crate::Result<String> {
        self.out = "global _start\n_start:\n".to_string();
        for node in root.statements {
            self.compile_call(node)?;
        }

        Ok(self.out)
    }

    fn compile_call(&mut self, node: NodeCall) -> crate::Result<()> {
        match node.identifier.as_str() {
            "exit" => {
                let int = match node.argument.literal {
                    Literal::Integer(int) => int,
                    _ => {
                        return Err(Error::new(
                            ErrorKind::UnexpectedType("Integer"),
                            node.argument.position,
                        ))
                    }
                };

                writeln!(&mut self.out, "mov rax, 0x3c").ok();
                writeln!(&mut self.out, "mov rdi, {int}").ok();
                writeln!(&mut self.out, "syscall").ok();
            }

            _ => {
                return Err(Error::new(
                    ErrorKind::NotFound(node.identifier),
                    node.identifier_position,
                ));
            }
        }

        Ok(())
    }
}
