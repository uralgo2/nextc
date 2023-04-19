use std::fmt::{Display, Formatter};
use crate::lexer::token::Token;

pub enum Node {
    Program(Vec<Statement>)
}

pub enum Statement {
    ExpressionStatement(Expression),
    LocalStatement(String, Option<String>, Option<Expression>),
    ReturnStatement,
    Empty
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Expression {
    BinaryExpression(Token, Box<Expression>, Box<Expression>),
    AssignmentExpression(Token, Box<Expression>, Box<Expression>),
    UnaryExpression(Token, Box<Expression>),
    DotExpression(Box<Expression>, Box<Expression>),
    CallExpression(Box<Expression>, Vec<Expression>),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    IdLiteral(String),
}

fn write_expr(expr: &Expression, depth: u32, pad: &str) {
    print!("{}", pad.repeat(depth as usize));

    match expr {
        Expression::BinaryExpression(op, lhs, rhs) => {
            let _op = if let Token::Operator(_op, ..) = op { _op } else { unreachable!() };
            println!("BinaryExpression \"{}\"", _op);

            write_expr(lhs, depth + 1, pad);
            write_expr(rhs, depth + 1, pad);
        }
        Expression::UnaryExpression(op, expr) => {
            let _op = if let Token::Operator(_op, ..) = op { _op } else { unreachable!() };
            println!("UnaryExpression \"{}\"", _op);

            write_expr(expr, depth + 1, pad);
        }
        Expression::IntegerLiteral(num) => {
            println!("IntegerExpression \"{}\"", num);
        }
        Expression::FloatLiteral(num) => {
            println!("FloatExpression \"{}\"", num);
        }
        Expression::IdLiteral(id) => {
            println!("IdExpression \"{}\"", id);
        }
         _ => unreachable!()
    }
}
fn write_st(st: &Statement, depth: u32, pad: &str) {
    print!("{}", pad.repeat(depth as usize));

    match st {
        Statement::LocalStatement(id, _type, expr) => {
            println!("LocalStatement id = \"{}\" type = \"{:?}\"", id, _type);
            if let Some(expr) = expr {
                write_expr(expr, depth + 1, pad);
            }
        }
        Statement::ExpressionStatement(expr) => {
            println!("ExpressionStatement");
            write_expr(expr, depth + 1, pad);
        }
        _ => unreachable!()
    }
}

impl Display for Statement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write_st(self, 0, "--");

        return Ok(());
    }
}
impl Display for Expression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write_expr(self, 0, "--");

        return Ok(());
    }
}