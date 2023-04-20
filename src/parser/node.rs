use crate::lexer::token::Token;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Param {
    pub(crate) name: String,
    pub(crate) _type: Option<String>,
}

pub enum Node {
    Program(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub struct ConditionBranch {
    pub expression: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    ExpressionStatement(Expression),
    LocalStatement(String, Option<String>, Option<Expression>),
    ConstantStatement(String, Option<String>, Expression),
    FunctionDeclarationStatement(String, Vec<Param>, Option<String>, Vec<Statement>),
    ReturnStatement(Option<Expression>),
    ImportStatement(String),
    UseStatement(String),
    DecoratedStatement(Vec<Expression>, Box<Statement>),
    IfStatement(
        ConditionBranch,
        Vec<ConditionBranch>,
        Option<Vec<Statement>>,
    ),
    ForStatement(
        Option<Box<Statement>>,
        Option<Box<Statement>>,
        Option<Box<Statement>>,
        Vec<Statement>,
    ),
    While(Expression, Vec<Statement>),
    EmptyStatement,
}

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryExpression(Token, Box<Expression>, Box<Expression>),
    AssignmentExpression(Token, Box<Expression>, Box<Expression>),
    PreUnaryExpression(Token, Box<Expression>),
    PostUnaryExpression(Token, Box<Expression>),
    DotExpression(Box<Expression>, Box<Expression>),
    CallExpression(Box<Expression>, Vec<Expression>),
    FunctionExpression(
        Vec<Param>,
        Option<String>,
        Option<Box<Expression>>,
        Vec<Statement>,
    ),
    IntegerLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    IdLiteral(String),
}

fn write_expr(expr: &Expression, depth: u32, pad: &str) {
    print!("{}", pad.repeat(depth as usize));

    match expr {
        Expression::BinaryExpression(op, lhs, rhs) => {
            let _op = if let Token::Operator(_op, ..) = op {
                _op
            } else {
                unreachable!()
            };
            println!("BinaryExpression \"{}\"", _op);

            write_expr(lhs, depth + 1, pad);
            write_expr(rhs, depth + 1, pad);
        }
        Expression::AssignmentExpression(op, lhs, rhs) => {
            let _op = if let Token::Operator(_op, ..) = op {
                _op
            } else {
                unreachable!()
            };
            println!("AssignmentExpression \"{}\"", _op);

            write_expr(lhs, depth + 1, pad);
            write_expr(rhs, depth + 1, pad);
        }
        Expression::DotExpression(lhs, rhs) => {
            println!("DotExpression");

            write_expr(lhs, depth + 1, pad);
            write_expr(rhs, depth + 1, pad);
        }
        Expression::PreUnaryExpression(op, expr) => {
            let _op = if let Token::Operator(_op, ..) = op {
                _op
            } else {
                unreachable!()
            };
            println!("PreUnaryExpression \"{}\"", _op);

            write_expr(expr, depth + 1, pad);
        }
        Expression::PostUnaryExpression(op, expr) => {
            let _op = if let Token::Operator(_op, ..) = op {
                _op
            } else {
                unreachable!()
            };
            println!("PostUnaryExpression \"{}\"", _op);

            write_expr(expr, depth + 1, pad);
        }
        Expression::IntegerLiteral(num) => {
            println!("IntegerExpression \"{}\"", num);
        }
        Expression::StringLiteral(str) => {
            println!("StringLiteral \"{}\"", str);
        }
        Expression::FloatLiteral(num) => {
            println!("FloatExpression \"{}\"", num);
        }
        Expression::IdLiteral(id) => {
            println!("IdExpression \"{}\"", id);
        }
        Expression::CallExpression(callable, args) => {
            println!("CallExpression");

            write_expr(callable, depth + 1, pad);

            for arg in args {
                write_expr(arg, depth + 1, pad);
            }
        }
        Expression::FunctionExpression(params, returnType, expr, body) => {
            print!("FunctionExpression params = ( ");

            for param in params {
                print!("{:?} ", param);
            }

            println!(") return type = \"{:?}\"", returnType);

            for statement in body {
                write_st(statement, depth + 1, pad);
            }
            if let Some(_expr) = expr {
                write_expr(_expr, depth + 1, pad);
            }
        }
        _ => unreachable!(),
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
        Statement::ConstantStatement(id, _type, expr) => {
            println!("ConstantStatement id = \"{}\" type = \"{:?}\"", id, _type);
            write_expr(expr, depth + 1, pad);
        }
        Statement::ImportStatement(id) => {
            println!("ImportStatement id = \"{}\"", id);
        }
        Statement::UseStatement(id) => {
            println!("UseStatement id = \"{}\"", id);
        }
        Statement::ExpressionStatement(expr) => {
            println!("ExpressionStatement");
            write_expr(expr, depth + 1, pad);
        }
        Statement::ReturnStatement(expr) => {
            println!("ReturnStatement");
            if expr.is_some() {
                write_expr(&(expr.clone().unwrap().clone()), depth + 1, pad);
            }
        }
        Statement::FunctionDeclarationStatement(name, params, returnType, body) => {
            print!("FunctionDeclarationStatement id = \"{}\" params = ( ", name);

            for param in params {
                print!("{:?} ", param);
            }

            println!(") return type = \"{:?}\"", returnType);

            for statement in body {
                write_st(statement, depth + 1, pad);
            }
        }
        Statement::ForStatement(init, cond, action, body) => {
            println!("ForStatement");

            if let Some(statement) = init.clone() {
                write_st(statement.as_ref(), depth + 1, pad);
            }
            if let Some(statement) = cond.clone() {
                write_st(statement.as_ref(), depth + 1, pad);
            }
            if let Some(statement) = action.clone() {
                write_st(statement.as_ref(), depth + 1, pad);
            }

            for statement in body {
                write_st(statement, depth + 1, pad);
            }
        }
        Statement::DecoratedStatement(decorators, statement) => {
            println!("DecoratedStatement");

            for decorator in decorators {
                write_expr(decorator, depth + 1, pad);
            }
            write_st(statement, depth + 1, pad);
        }
        Statement::EmptyStatement => {
            println!("EmptyStatement");
        }
        _ => unreachable!(),
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
