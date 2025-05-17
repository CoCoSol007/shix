//! The abstract syntax tree of the shix programming language

use num_bigint::BigInt;

/// An expression that represents a number
/// for example it could be :
/// 5, -6, (1 + 2) * 3 or pop * 3
#[derive(Debug, Clone)]
pub enum Expression {
    /// The fundamental unit of a number
    Number(BigInt),

    /// The pop keyword, enable to get the value on top of
    /// the stack and remove it
    Pop,

    /// The read keyword, enable to get the value on top of the stack
    Read,

    /// The addition operator a + b
    Addition(Box<Expression>, Box<Expression>),

    /// The substraction operator a - b
    Substract(Box<Expression>, Box<Expression>),

    /// The negate operator -a
    Negate(Box<Expression>),

    /// The multiplication operator a * b
    Multiply(Box<Expression>, Box<Expression>),

    /// An error
    Error(String),
}

/// A jump instruction
#[derive(Debug, Clone)]
pub enum Jump {
    /// Jump to the line n if the element given is 0
    JumpZero,

    /// Jump to the line n if the element given is not 0
    JumpNotZero,

    /// Jump to the line n if the element given is < 0
    JumpNegate,

    /// Jump to the line n if the element given is > 0
    JumpPositive,
}

/// A statement, the main unit of the AST
#[derive(Debug, Clone)]
pub enum Statement {
    /// An expression
    Expression(Expression),

    /// A push instruction
    Push(Expression),

    /// A print instruction
    Print(Expression),

    /// A jump instruction
    Jump {
        /// The line to jump
        line: Expression,

        /// The value to compare
        value: Expression,

        /// The jump instruction
        jump: Jump,
    },

    /// A swap instruction, swap the two values on top of the stack
    Swap,

    /// A duplicate instruction, duplicate the n value on top of the stack
    /// Not very memory efficient
    Over(Expression),

    /// Clear the stack
    Clear,

    /// An empty line
    None,
}

/// A line of the program
pub struct Line {
    /// The statement of the line
    pub statement: Statement,

    /// The line number
    pub line_number: usize,
}
