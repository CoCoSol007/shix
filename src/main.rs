//! The main file of the shix programming language.

pub mod ast;
pub mod eval;
pub mod parser;

use std::collections::LinkedList;
use std::sync::RwLock;
use std::{env, fs};

use chumsky::Parser;

use crate::eval::eval_statement;
use crate::parser::parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("You must provide filecode.");
    }
    let src = fs::read_to_string(args[1].clone()).expect("Cannot read file for some reasons");

    let stack = RwLock::new(LinkedList::new());
    let ast = parser().parse(&src).into_result();

    if let Ok(ast) = ast.clone() {
        let ast_to_interpret = ast
            .into_iter()
            .filter(|a| !matches!(a, ast::Statement::None))
            .collect::<Vec<_>>();

        // Use the current line for jump
        let mut current_line = 0;

        // Main loop
        while current_line < ast_to_interpret.len() {
            let expr = &ast_to_interpret[current_line];
            current_line += 1;
            match eval_statement(expr, &stack, &mut current_line) {
                Ok(_) => (),
                Err(eval_err) => {
                    println!("Evaluation error: {eval_err} at line {current_line}");
                    break;
                }
            }
        }
    }
    if let Err(err) = ast {
        println!("Parsing error: {err:?}");
    }
}
