use std::{collections::HashMap, fmt::Write};

use crate::{Literal, NodeExpr, NodeProgram, NodeStmt, NodeStmtCall};

pub struct Variable {
    stack_location: usize,
}

#[derive(Default)]
pub struct AsmGenerator {
    out: String,
    variables: HashMap<String, Variable>,
    stack_size: usize,
}

impl AsmGenerator {
    pub fn generate(mut self, program: NodeProgram) -> crate::Result<String> {
        self.out = "global _start\n_start:\n".to_string();
        for stmt in program.statements {
            self.generate_stmt(stmt)?;
        }

        Ok(self.out)
    }

    fn generate_stmt(&mut self, stmt: NodeStmt) -> crate::Result<()> {
        match stmt {
            NodeStmt::Call(call) => {
                self.generate_call(call)?;
            }
            NodeStmt::Let(let_stmt) => {
                if self.variables.contains_key(&let_stmt.identifier.0) {
                    Err(let_stmt.identifier.make_already_used_error())?;
                }

                self.variables.insert(
                    let_stmt.identifier.0,
                    Variable {
                        stack_location: self.stack_size,
                    },
                );
                self.generate_expr(let_stmt.expression)?;
            }
        }
        Ok(())
    }

    fn generate_call(&mut self, call: NodeStmtCall) -> crate::Result<()> {
        match call.identifier.0.as_str() {
            "exit" => {
                self.generate_expr(call.argument)?;
                // syscall for exit
                self.write("mov rax, 0x3c");
                self.generate_pop("rdi");
                self.write("syscall");
            }
            _ => Err(call.identifier.make_not_found_error())?,
        }

        Ok(())
    }

    fn generate_expr(&mut self, expr: NodeExpr) -> crate::Result<()> {
        match expr {
            NodeExpr::Literal(literal) => match literal.0 {
                Literal::Integer(integer) => self.generate_push(integer.to_string()),
                Literal::String(_) => unimplemented!(),
            },
            NodeExpr::Identifer(identifer) => {
                let variable = self
                    .variables
                    .get(&identifer.0)
                    .ok_or(identifer.make_not_found_error())?;
                self.generate_push(format!("QWORD [rsp + {}]", variable.stack_location))
            }
        }
        Ok(())
    }

    fn generate_push(&mut self, value: impl AsRef<str>) {
        let size = 8; // currently only 64-bit integers supported
        self.write(format!("push {}", value.as_ref()));
        self.stack_size += size;
    }

    fn generate_pop(&mut self, value: &str) {
        let size = 8; // currently only 64-bit integers supported
        self.write(format!("pop {value}"));
        self.stack_size -= size;
    }

    fn write(&mut self, text: impl AsRef<str>) {
        // Writing to a string can never fail
        writeln!(&mut self.out, "{}", text.as_ref()).ok();
    }
}
