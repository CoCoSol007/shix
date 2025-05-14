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

    // Remove comments
    let src = src
        .lines()
        .map(|line| line.split("//").next().unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n");

    // Remove empty lines
    let src = src
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");

    let stack = RwLock::new(LinkedList::new());

    let Ok(ast) = parser().parse(&src).into_result() else {
        println!("Syntax error");
        return;
    };

    // Use the current line for jump
    let mut current_line = 0;

    // Main loop
    while current_line < ast.len() {
        let expr = &ast[current_line].statement;
        current_line += 1;
        match eval_statement(expr, &stack, &mut current_line) {
            Ok(_) => (),
            Err(eval_err) => {
                println!("Evaluation error: {eval_err} at line {}", current_line + 1);
                break;
            }
        }
    }
}
