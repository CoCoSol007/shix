//! The parser of the shix programming language

use chumsky::prelude::*;
use num_bigint::BigInt;

use crate::ast::*;

/// The parser of the shix programming language
pub fn parser<'src>() -> impl Parser<'src, &'src str, Vec<Statement>> {
    parser_line().repeated().collect().then_ignore(end())
}

/// The parser of a single line of the shix programming language
pub fn parser_line<'src>() -> impl Parser<'src, &'src str, Statement> {
    recursive(|expr| {
        let int = text::int(10).map(|str_number: &str| {
            let Ok(number) = str_number.parse::<BigInt>() else {
                return Statement::Expression(Expression::Error(format!(
                    "Unable to parse the number : {str_number}"
                )));
            };
            Statement::Expression(Expression::Number(number))
        });

        let int_or_pop = int
            .or(just("pop").map(|_| Statement::Expression(Expression::Pop)))
            .or(just("read").map(|_| Statement::Expression(Expression::Read)));

        let atom = int_or_pop
            .or(expr.clone().delimited_by(
                just('('),
                just(')').map_err(|e: EmptyErr| {
                    println!("Expected a closing parenthesis");
                    e
                }),
            ))
            .padded();

        let op = |c| just(c).padded();

        let unary = op("-").repeated().foldr(atom, |_op, rhs| {
            let expr_rhs: Expression = match rhs {
                Statement::Expression(expression) => expression,
                _ => Expression::Error("rhs is not an expression".to_string()),
            };
            Statement::Expression(Expression::Negate(Box::new(expr_rhs)))
        });

        let product = unary.clone().foldl(
            choice((
                op("*").to(Expression::Multiply as fn(_, _) -> _),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| {
                let lhs_expr = match lhs {
                    Statement::Expression(expr) => expr,
                    _ => Expression::Error("lhs is not an expression".to_string()),
                };
                let rhs_expr = match rhs {
                    Statement::Expression(expr) => expr,
                    _ => Expression::Error("rhs is not an expression".to_string()),
                };
                Statement::Expression(op(Box::new(lhs_expr), Box::new(rhs_expr)))
            },
        );

        let sum = product.clone().foldl(
            choice((
                op("+").to(Expression::Addition as fn(_, _) -> _),
                op("-").to(Expression::Substract as fn(_, _) -> _),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| {
                let lhs_expr = match lhs {
                    Statement::Expression(expr) => expr,
                    _ => Expression::Error("lhs is not an expression".to_string()),
                };
                let rhs_expr = match rhs {
                    Statement::Expression(expr) => expr,
                    _ => Expression::Error("rhs is not an expression".to_string()),
                };
                Statement::Expression(op(Box::new(lhs_expr), Box::new(rhs_expr)))
            },
        );

        let push_operation = op("push")
            .then_ignore(just(":").map_err(|e: EmptyErr| {
                println!("Expected ':' after 'push'");
                e
            }))
            .then(expr.clone().map_err(|e: EmptyErr| {
                println!("Expected expression after ':'");
                e
            }))
            .map(|(_, expr)| match expr {
                Statement::Expression(expr) => Statement::Push(expr),
                _ => {
                    Statement::Expression(Expression::Error("rhs is not an expression".to_string()))
                }
            });

        let print_operation = op("print")
            .then_ignore(just(":").map_err(|e: EmptyErr| {
                println!("Expected ':' after 'print'");
                e
            }))
            .then(expr.clone().map_err(|e: EmptyErr| {
                println!("Expected expression after ':'");
                e
            }))
            .map(|(_, expr)| match expr {
                Statement::Expression(expr) => Statement::Print(expr),
                _ => {
                    Statement::Expression(Expression::Error("rhs is not an expression".to_string()))
                }
            });

        let swap_operation = op("swap").to(Statement::Swap);
        let clear_operation = op("clear").to(Statement::Clear);

        let over_operation = op("over")
            .then_ignore(just(":").map_err(|e: EmptyErr| {
                println!("Expected ':' after 'over'");
                e
            }))
            .then(expr.clone().map_err(|e: EmptyErr| {
                println!("Expected expression after ':'");
                e
            }))
            .map(|(_, expr)| match expr {
                Statement::Expression(expr) => Statement::Over(expr),
                _ => {
                    Statement::Expression(Expression::Error("rhs is not an expression".to_string()))
                }
            });

        let special_jump_operation = |jump: &'static str| {
            op(jump)
                .then_ignore(just(":").map_err(move |e: EmptyErr| {
                    println!("Expected ':' after '{jump}'");
                    e
                }))
                .then(expr.clone().map_err(move |e: EmptyErr| {
                    let jump_type = match jump {
                        "jump" => "target line",
                        _ => "condition",
                    };
                    println!("Expected expression ({jump_type}) after ':'");
                    e
                }))
                .then_ignore(just(",").map_err(move |e: EmptyErr| {
                    println!("Expected ',' after the condition for '{jump}'");
                    e
                }))
                .then(expr.clone().map_err(|e: EmptyErr| {
                    println!("Expected expression (target line) after ','");
                    e
                }))
                .map(move |((_push, condition), line)| {
                    let Statement::Expression(condition) = condition else {
                        return Statement::Expression(Expression::Error(
                            "first argument is not an expression".to_string(),
                        ));
                    };
                    let Statement::Expression(line) = line else {
                        return Statement::Expression(Expression::Error(
                            "rhs is not an expression".to_string(),
                        ));
                    };
                    let jump_type = match jump {
                        "jumpZ" => Jump::JumpZero,
                        "jumpNZ" => Jump::JumpNotZero,
                        "jumpN" => Jump::JumpNegate,
                        "jumpP" => Jump::JumpPositive,
                        _ => panic!("Unknown command {jump}"),
                    };
                    Statement::Jump {
                        line,
                        value: condition,
                        jump: jump_type,
                    }
                })
        };

        let comment = just("//")
            .then(any().and_is(just('\n').not()).repeated())
            .padded()
            .to(Statement::None);

        choice((
            comment,
            sum,
            push_operation,
            print_operation,
            swap_operation,
            clear_operation,
            over_operation,
            special_jump_operation("jumpZ"),
            special_jump_operation("jumpNZ"),
            special_jump_operation("jumpN"),
            special_jump_operation("jumpP"),
        ))
    })
}
