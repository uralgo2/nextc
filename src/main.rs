use crate::compiler::functions::FunctionTable;
use crate::compiler::next_to_bytecode_compiler::NextToByteCodeCompiler;
use crate::compiler::types::TypeTable;
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use crate::parser::node::Expression;
use crate::parser::Parser;
use std::collections::HashMap;
use std::io::Write;
use std::{env, fs, io};

mod compiler;
mod core;
mod lexer;
mod parser;

fn main() {
    let keywords = vec![
        String::from("let"),
        String::from("const"),
        String::from("fn"),
        String::from("import"),
        String::from("export"),
        String::from("return"),
        String::from("use"),
        String::from("for"),
    ];
    let operators = vec![
        String::from("+"),
        String::from("-"),
        String::from("*"),
        String::from("**"),
        String::from("/"),
        String::from("%"),
        String::from("^"),
        String::from("&"),
        String::from("="),
        String::from('.'),
        String::from("+="),
        String::from("-="),
        String::from("*="),
        String::from("/="),
        String::from("%="),
        String::from("**="),
        String::from("++"),
        String::from("--"),
        String::from("!"),
        String::from("~"),
        String::from("<"),
        String::from(">"),
        String::from("<="),
        String::from(">="),
        String::from("=="),
        String::from("!="),
    ];
    let specials = vec![
        String::from("{"),
        String::from("}"),
        String::from("("),
        String::from(")"),
        String::from("["),
        String::from("]"),
        String::from("|"),
        String::from("|"),
        String::from(":"),
        String::from(";"),
        String::from("=>"),
        String::from("->"),
        String::from("@"),
        String::from(","),
    ];
    let operators_priorities = HashMap::from([
        (String::from("+"), 1),
        (String::from("-"), 1),
        (String::from("*"), 2),
        (String::from("/"), 2),
        (String::from("%"), 2),
        (String::from("**"), 3),
        (String::from("."), 4),
    ]);
    let binary_operators = vec![
        String::from("+"),
        String::from("-"),
        String::from("*"),
        String::from("**"),
        String::from("/"),
        String::from("%"),
        String::from("."),
    ];
    let unary_operators = vec![
        String::from("+"),
        String::from("-"),
        String::from("++"),
        String::from("--"),
        String::from("!"),
        String::from("~"),
    ];

    let args: Vec<_> = env::args().collect();

    let mut input = String::from("1 + 2 + 3 * 3");

    if args.get(1).is_some() {
        let file_contents = fs::read_to_string(
            args.get(1)
                .expect("You should pass file name as first argument"),
        )
        .expect("Should have been able to read the file");

        input = file_contents;

        let mut lexer = Lexer::new(
            input.chars().collect(),
            keywords.clone(),
            operators.clone(),
            specials.clone(),
        );
        lexer.read_char();
        loop {
            let tok = lexer.next_token();

            //println!("{:?}", tok);

            if tok == Token::EOF {
                break;
            }
        }

        let mut parser = Parser::new(
            lexer,
            operators_priorities.clone(),
            binary_operators.clone(),
            unary_operators.clone(),
        );

        let program = parser.parse();

        for st in &program {
            println!("{}", st);
        }

        let mut compiler = NextToByteCodeCompiler {
            context: vec![],
            types: TypeTable { types: vec![] },
            globals: Default::default(),
            top_level_functions: FunctionTable { functions: vec![] },
        };

        let compiled_ctx = compiler.compile_program(program);

        for fun in &(*compiled_ctx).borrow().functions.functions {
            println!("{}", fun.borrow());
        }
        return;
    }

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("ERROR");

        let mut lexer = Lexer::new(
            input.chars().collect(),
            keywords.clone(),
            operators.clone(),
            specials.clone(),
        );
        lexer.read_char();
        loop {
            let tok = lexer.next_token();

            //println!("{:?}", tok);

            if tok == Token::EOF {
                break;
            }
        }

        let mut parser = Parser::new(
            lexer,
            operators_priorities.clone(),
            binary_operators.clone(),
            unary_operators.clone(),
        );

        let program = parser.parse();

        for st in &program {
            println!("{}", st);
        }
        let mut compiler = NextToByteCodeCompiler {
            context: vec![],
            types: TypeTable { types: vec![] },
            globals: Default::default(),
            top_level_functions: FunctionTable { functions: vec![] },
        };

        let compiled_ctx = compiler.compile_program(program);

        println!("Imports: {:?}", (*compiled_ctx).borrow().imports.imports);

        for fun in &(*compiled_ctx).borrow().functions.functions {
            println!("{}", fun.borrow());
        }
    }
}

fn eval(expr: Expression) -> f64 {
    match expr {
        Expression::BinaryExpression(op, lhs, rhs) => {
            let lhs_value = eval(*lhs);
            let rhs_value = eval(*rhs);
            let _op = if let Token::Operator(_op, ..) = op {
                _op
            } else {
                unreachable!()
            };
            return match &*_op {
                "+" => lhs_value + rhs_value,
                "-" => lhs_value - rhs_value,
                "*" => lhs_value * rhs_value,
                "**" => lhs_value.powf(rhs_value),
                "/" => lhs_value / rhs_value,
                "%" => lhs_value % rhs_value,
                _ => unreachable!(),
            };
        }
        Expression::PreUnaryExpression(op, expr) => {
            let expr_value = eval(*expr);

            match op {
                Token::Operator(_op, ..) => {
                    return if _op == "+" { expr_value } else { -expr_value }
                }
                _ => unreachable!(),
            };
        }
        Expression::IntegerLiteral(num) => {
            return num as f64;
        }
        Expression::FloatLiteral(num) => {
            return num;
        }
        _ => unreachable!(),
    };
}
