//! The abstract syntax tree of the shix programming language

/// An expression that represents a number
/// for example it could be :
/// 5, -6, (1 + 2) * 3 or pop * 3
#[derive(Debug, Clone)]
pub enum Expression {
    /// The fundamental unit of a number
    Number(f64),

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

    /// The division operator a / b
    Divide(Box<Expression>, Box<Expression>),

    /// The modulo operator a % b
    Modulo(Box<Expression>, Box<Expression>),

    /// An error
    Error(String),
}

/// A jump instruction
#[derive(Debug, Clone)]
pub enum Jump {
    /// Jump to the line n if the element given is 0
    JumpZero(Expression),

    /// Jump to the line n if the element given is not 0
    JumpNotZero(Expression),

    /// Jump to the line n if the element given is < 0
    JumpNegate(Expression),

    /// Jump to the line n if the element given is > 0
    JumpPositive(Expression),
}

impl Jump {
    /// Get the command of the jump
    pub fn get_command(self) -> String {
        match self {
            Self::JumpZero(_) => "jumpZ".to_string(),
            Self::JumpNotZero(_) => "jumpNZ".to_string(),
            Self::JumpNegate(_) => "jumpN".to_string(),
            Self::JumpPositive(_) => "jumpP".to_string(),
        }
    }

    /// Create a jump instruction from a command
    pub fn from_command(command: &'static str, expr: Expression) -> Self {
        match command {
            "jumpZ" => Self::JumpZero(expr),
            "jumpNZ" => Self::JumpNotZero(expr),
            "jumpN" => Self::JumpNegate(expr),
            "jumpP" => Self::JumpPositive(expr),
            _ => panic!("Unknown command {command}"),
        }
    }
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
    Jump(Expression, Jump),

    /// A swap instruction, swap the two values on top of the stack
    Swap,

    /// A duplicate instruction, duplicate the n value on top of the stack
    /// Not very memory efficient
    Over(Expression),

    /// Clear the stack
    Clear,
}

/// A line of the program
pub struct Line {
    /// The statement of the line
    pub statement: Statement,

    /// The line number
    pub line_number: usize,
}
