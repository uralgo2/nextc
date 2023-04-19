use std::collections::HashMap;
use crate::lexer::Lexer;
use crate::lexer::token::Token;
use crate::parser::node::{Expression, Statement};

pub mod node;

pub struct Parser {
    lexer: Lexer,
    operators: HashMap<String, u16>,
    unary_operators: Vec<String>,
    binary_operators: Vec<String>
}

impl Parser {
    pub fn new(lexer: Lexer, operators: HashMap<String, u16>,  binary_operators: Vec<String>, unary_operators: Vec<String>) -> Self {
        Self {
            lexer,
            operators,
            binary_operators,
            unary_operators
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        self.lexer.reset();

        self.lexer.read_char();

        let mut statements = vec![];

        while self.lexer.peek_token() != Token::EOF {
            statements.push(self.parse_statement());
        }
        return statements;
    }

    pub fn parse_statement(&mut self) -> Statement {
        let tok = self.lexer.peek_token();

        match tok.clone() {
            Token::Keyword(keyword, ..) => {
                if keyword == "let" {
                    self.parse_local()
                }
                else {
                    panic!("Undefined keyword")
                }
            }
            Token::Operator(..) => {
                Statement::ExpressionStatement(self.parse_expression())
            }
            Token::Integer(..) => {
                Statement::ExpressionStatement(self.parse_expression())
            }
            Token::Float(..) => {
                Statement::ExpressionStatement(self.parse_expression())
            }
            Token::Id(..) => {
                Statement::ExpressionStatement(self.parse_expression())
            }
            Token::EOF => Statement::Empty,
            Token::Unknown(_u, ..) => panic!("Unknown token {:?}", tok.clone()),
            _ => unreachable!()
        }
    }

    pub fn parse_local(&mut self) -> Statement {
        self.lexer.next_token(); // let
        let id_tok = self.lexer.next_token();

        let id = if let Token::Id(_id, ..) = id_tok {
            _id
        } else {
            panic!("Expected identifier but got {:?}", id_tok);
        };

        let _type: Option<String>;
        let mut expr: Option<Expression> = None;

        let next = self.lexer.peek_token();
        match next.clone() {
            Token::Operator(_op, _) => {
                let equal_tok = self.lexer.next_token();

                if _op != "=" {
                    panic!("Expected assignment operator but got {:?}", equal_tok.clone());
                }

                expr = Some(self.parse_expression());

                _type = None;
            }
            Token::Special(sp, _) => {
                let colon_tok = self.lexer.next_token();

                let type_tok = self.lexer.next_token();

                if sp != ":" {
                    panic!("Expected colon but got {:?}", colon_tok.clone());
                }

                if let Token::Id(name, ..) = type_tok {
                    _type = Some(name);
                }
                else {
                    panic!("Expected type name but got {:?}", type_tok.clone());
                }
            }
            _ => _type = None
        }

        if _type != None {
            let next = self.lexer.peek_token();
            match next.clone() {
                Token::Operator(_op, _) => {
                    let equal_tok = self.lexer.next_token();

                    if _op != "=" {
                        panic!("Expected assignment operator but got {:?}", equal_tok.clone());
                    }

                    expr = Some(self.parse_expression());
                }
                _ => expr = None
            }
        }

        return Statement::LocalStatement(id, _type,expr);
    }
    pub fn parse_expression(&mut self) -> Expression {
        let mut output_tokens: Vec<(Token, bool)> = vec![];
        let mut stack: Vec<(Token, bool)> = vec![];

        let mut prev: Option<Token> = None;

        loop {
            let symbol = self.lexer.next_token();
            match symbol.clone() {
                Token::Integer(_, _) => {
                    if let Some(tok) = prev.clone() {
                        if let Token::Operator(..) = tok.clone() {

                        }
                        else {
                            panic!("Expected operator but got {:?}", tok.clone())
                        }
                    }

                    output_tokens.push((symbol.clone(), false));
                }
                Token::Float(_, _) => {output_tokens.push((symbol.clone(), false));}
                Token::Id(_, _) => {output_tokens.push((symbol.clone(), false));}
                Token::Operator(_op, _) => {
                    let priority = self.operators.get(&_op).expect("");

                    if let Some(Token::Operator(_op, ..)) = prev {
                        stack.push((symbol.clone(), true));
                    }
                    else if prev == None {
                        stack.push((symbol.clone(), true));
                    }
                    else {
                        if let Some((Token::Operator(_op, ..), ..)) = stack.last() {
                            if priority <= self.operators.get(&_op.clone()).expect("") {
                                output_tokens.push(stack.pop().unwrap());
                            }
                        }

                        stack.push((symbol.clone(), false));
                    }
                }
                Token::Special(ch, _) => {
                    if ch == "(" {
                        stack.push((symbol.clone(), false));
                    }
                    else if ch == ")" {
                        loop {
                            let tok = stack.pop().expect("Invalid format");

                            if let Token::Special(_, ..) = tok.clone().0 {
                                break;
                            }
                            else {
                                output_tokens.push(tok.clone())
                            }
                        }
                    }
                    else {
                        break;
                    }
                }
                _ => break
            }
            prev = Some(symbol.clone());
        }

        while stack.len() != 0 {
            output_tokens.push(stack.pop().unwrap());
        }

        let mut stack: Vec<Expression> = vec![];

        for tok in output_tokens {
            match tok.0.clone() {
                Token::Integer(num, _) => {
                    stack.push(Expression::IntegerLiteral(num));
                }
                Token::Float(num, _) => {
                    stack.push(Expression::FloatLiteral(num));
                }
                Token::Id(id, _) => {
                    stack.push(Expression::IdLiteral(id));
                }
                Token::Operator(op, _) => {
                    let s = stack.pop().unwrap();
                    if !tok.1 {
                        let f = stack.pop().unwrap();

                        stack.push(Expression::BinaryExpression(tok.0.clone(), Box::new(f), Box::new(s)));
                    }
                    else {
                        stack.push(Expression::UnaryExpression(tok.0.clone(), Box::new(s)));
                    }
                }
                _ => unreachable!()
            }
        }

        return stack.pop().unwrap();
    }
}
