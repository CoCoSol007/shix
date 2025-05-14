//! The parser of the shix programming language

use chumsky::prelude::*;

use crate::ast::*;

/// The parser of the shix programming language
pub fn parser<'src>() -> impl Parser<'src, &'src str, Vec<Line>> {
    parser_line()
        .map_with(|stmt, span| Line {
            statement: stmt,
            line_number: span.span().start,
        })
        .then_ignore(just("\n").or_not())
        .repeated()
        .collect()
        .then_ignore(end())
}

/// The parser of a single line of the shix programming language
pub fn parser_line<'src>() -> impl Parser<'src, &'src str, Statement> {
    recursive(|expr| {
        let int = text::int(10).map(|str_number: &str| {
            let Ok(number) = str_number.parse::<f64>() else {
                return Statement::Expression(Expression::Error(format!(
                    "Unable to parse the number : {str_number}"
                )));
            };
            Statement::Expression(Expression::Number(number))
        });

        let float = text::int(10)
            .then_ignore(just("."))
            .then(text::int(10).map_err(|e: EmptyErr| {
                println!("Expected a number after the dot");
                e
            }))
            .map(|(a, b): (&str, &str)| {
                let Ok(number) = format!("{a}.{b}").parse::<f64>() else {
                    return Statement::Expression(Expression::Error(format!(
                        "Unable to parse the number : {a}.{b}"
                    )));
                };
                Statement::Expression(Expression::Number(number))
            });

        let int_or_pop = float
            .or(int)
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

        let product =
            unary.clone().foldl(
                choice((
                    op("*").to(Expression::Multiply
                        as fn(_: Box<Expression>, _: Box<Expression>) -> Expression),
                    op("/").to(Expression::Divide
                        as fn(_: Box<Expression>, _: Box<Expression>) -> Expression),
                    op("%").to(Expression::Modulo
                        as fn(_: Box<Expression>, _: Box<Expression>) -> Expression),
                ))
                .then(unary.map_err(|e: EmptyErr| {
                    println!("Expected an expression after the operator");
                    e
                }))
                .repeated(),
                |lhs, (op, rhs)| {
                    let lhs_expr: Expression = match lhs {
                        Statement::Expression(expression) => expression,
                        _ => Expression::Error("lhs is not an expression".to_string()),
                    };

                    let rhs_expr: Expression = match rhs {
                        Statement::Expression(expression) => expression,
                        _ => Expression::Error("rhs is not an expression".to_string()),
                    };

                    Statement::Expression(op(Box::new(lhs_expr), Box::new(rhs_expr)))
                },
            );

        let sum = product.clone().foldl(
            choice((
                op("+").to(Expression::Addition
                    as fn(_: Box<Expression>, _: Box<Expression>) -> Expression),
                op("-").to(Expression::Substract
                    as fn(_: Box<Expression>, _: Box<Expression>) -> Expression),
            ))
            .then(product.map_err(|e: EmptyErr| {
                println!("Expected an expression after the operator");
                e
            }))
            .repeated(),
            |lhs, (op, rhs)| {
                let lhs_expr: Expression = match lhs {
                    Statement::Expression(expression) => expression,
                    _ => Expression::Error("lhs is not an expression".to_string()),
                };

                let rhs_expr: Expression = match rhs {
                    Statement::Expression(expression) => expression,
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
            .map(|(_push, expr)| match expr {
                Statement::Expression(expression) => Statement::Push(expression),
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
            .map(|(_push, expr)| match expr {
                Statement::Expression(expression) => Statement::Print(expression),
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
            .map(|(_push, expr)| match expr {
                Statement::Expression(expression) => Statement::Over(expression),
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

                    Statement::Jump {
                        line,
                        value: condition,
                        jump: Jump::from_command(jump),
                    }
                })
        };

        choice((
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
