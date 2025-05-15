//! The evaluator of the shix programming language

use std::collections::LinkedList;
use std::sync::RwLock;

use crate::ast::*;

/// Evaluate a statement
pub fn eval_statement(
    statement: &Statement,
    stack: &RwLock<LinkedList<f64>>,
    line_number: &mut usize,
) -> Result<(), String> {
    match statement {
        Statement::Expression(expression) => {
            let Err(err) = eval_expr(expression, stack) else {
                return Ok(());
            };
            Err(err)
        }
        Statement::Print(expr) => {
            println!("{}", eval_expr(expr, stack)?);
            Ok(())
        }
        Statement::Push(expr) => {
            let expr_value = eval_expr(expr, stack)?;
            let Ok(mut lock_stack) = stack.write() else {
                return Err("Unable to write the stack".to_string());
            };
            lock_stack.push_front(expr_value);
            Ok(())
        }
        Statement::Jump { line, value, jump } => match jump {
            Jump::JumpZero => eval_jump(|a| a == 0.0, stack, line, line_number, value),
            Jump::JumpNotZero => eval_jump(|a| a != 0.0, stack, line, line_number, value),
            Jump::JumpNegate => eval_jump(|a| a < 0.0, stack, line, line_number, value),
            Jump::JumpPositive => eval_jump(|a| a > 0.0, stack, line, line_number, value),
        },
        Statement::Swap => {
            let Ok(mut lock_stack) = stack.write() else {
                return Err("Unable to write the stack".to_string());
            };

            let Some(first) = lock_stack.pop_front() else {
                return Err("Stack is empty".to_string());
            };
            let Some(second) = lock_stack.pop_front() else {
                return Err("Stack contains only one value".to_string());
            };

            lock_stack.push_front(first);
            lock_stack.push_front(second);
            Ok(())
        }
        Statement::Over(expr) => {
            let index = eval_expr(expr, stack)?;
            let Ok(mut lock_stack) = stack.write() else {
                return Err("Unable to write the stack".to_string());
            };
            let clone_stack = lock_stack.clone();
            for (i, n) in clone_stack.iter().enumerate() {
                if i as f64 == index {
                    lock_stack.push_front(*n);
                    return Ok(());
                }
            }
            Err("Index out of bound".to_string())
        }
        Statement::Clear => {
            let Ok(mut lock_stack) = stack.write() else {
                return Err("Unable to write the stack".to_string());
            };
            lock_stack.clear();
            Ok(())
        }
        Statement::None => {
            println!("None");
            Ok(())
        }
    }
}

/// Evaluate an expression
fn eval_expr(expr: &Expression, stack: &RwLock<LinkedList<f64>>) -> Result<f64, String> {
    match expr {
        Expression::Number(n) => Ok(*n),
        Expression::Pop => {
            let Ok(mut lock_stack) = stack.write() else {
                return Err("Unable to write the stack".to_string());
            };
            lock_stack
                .pop_front()
                .map_or_else(|| Err("Stack underflow".to_string()), Ok)
        }
        Expression::Addition(a, b) => Ok(eval_expr(a, stack)? + eval_expr(b, stack)?),
        Expression::Substract(a, b) => Ok(eval_expr(a, stack)? - eval_expr(b, stack)?),
        Expression::Multiply(a, b) => Ok(eval_expr(a, stack)? * eval_expr(b, stack)?),
        Expression::Divide(a, b) => Ok(eval_expr(a, stack)? / eval_expr(b, stack)?),
        Expression::Modulo(a, b) => Ok(eval_expr(a, stack)? % eval_expr(b, stack)?),
        Expression::Negate(expression) => Ok(-eval_expr(expression, stack)?),
        Expression::Read => {
            let Ok(read_stack) = stack.read() else {
                return Err("Unable to write the stack".to_string());
            };
            read_stack
                .front()
                .map_or_else(|| Err("Stack underflow".to_string()), Ok)
                .copied()
        }
        Expression::Error(e) => Err(e.clone()),
    }
}

/// Evaluate a jump
fn eval_jump(
    condition: fn(f64) -> bool,
    stack: &RwLock<LinkedList<f64>>,
    line: &Expression,
    current_line: &mut usize,
    value: &Expression,
) -> Result<(), String> {
    let eval_line = eval_expr(line, stack)?;
    let eval_value = eval_expr(value, stack)?;
    if !condition(eval_value) {
        return Ok(());
    }
    *current_line = eval_line as usize;
    Ok(())
}
