use crate::lexer::token::Token;
use crate::lexer::Lexer;
use crate::parser::node::{Expression, Param, Statement};
use std::collections::HashMap;

pub mod node;

pub struct Parser {
    lexer: Lexer,
    operators: HashMap<String, u16>,
    unary_operators: Vec<String>,
    binary_operators: Vec<String>,
}

impl Parser {
    pub fn new(
        lexer: Lexer,
        operators: HashMap<String, u16>,
        binary_operators: Vec<String>,
        unary_operators: Vec<String>,
    ) -> Self {
        Self {
            lexer,
            operators,
            binary_operators,
            unary_operators,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        self.lexer.reset();

        self.lexer.read_char();

        return self.parse_program();
    }

    pub fn parse_program(&mut self) -> Vec<Statement> {
        let mut program = vec![];

        loop {
            let next = self.lexer.peek_token();

            if let Token::EOF = next.clone() {
                self.lexer.next_token();
                break;
            } else if let Token::Special(sp, ..) = next.clone() {
                if sp == ";" {
                    self.lexer.next_token();
                    continue;
                }
            }
            let statement = self.parse_statement();

            //println!("{}", statement);
            program.push(statement);
        }

        return program;
    }

    pub fn parse_decorated_statement(&mut self) -> Statement {
        let mut decorators = vec![];

        loop {
            let next = self.lexer.peek_token();

            if let Token::Special(sp, ..) = next.clone() {
                if sp == "@" {
                    self.lexer.next_token();
                    decorators.push(self.parse_expression());
                } else {
                    panic!("Expected at but got {:?}", next.clone());
                }
            } else if let Token::Keyword(kw, ..) = next.clone() {
                return Statement::DecoratedStatement(decorators, Box::new(self.parse_statement()));
            } else {
                panic!("Expected at or keyword but got {:?}", next.clone());
            }
        }

        unreachable!()
    }

    pub fn parse_import_statement(&mut self) -> Statement {
        let mut import_name = String::new();

        self.lexer.next_token(); // import

        loop {
            let token = self.lexer.peek_token();

            if let Token::Id(id, ..) = token {
                import_name += &*id;
                self.lexer.next_token();

                let maybe_dot = self.lexer.peek_token();

                if let Token::Operator(op, ..) = maybe_dot {
                    if op == "." {
                        self.lexer.next_token();
                        import_name += ".";
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        return Statement::ImportStatement(import_name.clone());
    }
    pub fn parse_use_statement(&mut self) -> Statement {
        let mut use_name = String::new();

        self.lexer.next_token(); // import

        loop {
            let token = self.lexer.peek_token();

            if let Token::Id(id, ..) = token {
                use_name += &*id;
                self.lexer.next_token();

                let maybe_dot = self.lexer.peek_token();

                if let Token::Operator(op, ..) = maybe_dot {
                    if op == "." {
                        self.lexer.next_token();
                        use_name += ".";
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        return Statement::UseStatement(use_name.clone());
    }
    pub fn parse_for_statement(&mut self) -> Statement {
        self.lexer.next_token(); // for

        let mut init = None;
        let mut cond = None;
        let mut action = None;
        let mut body = vec![];

        let must_be_bracket = self.lexer.next_token();

        if let Token::Special(sp, ..) = must_be_bracket.clone() {
            if sp != "(" {
                panic!(
                    "Expected open bracket but got {:?}",
                    must_be_bracket.clone()
                );
            }
        } else {
            panic!(
                "Expected open bracket but got {:?}",
                must_be_bracket.clone()
            );
        }

        init = self.parse_statement_or_semicolon();

        self.expect_semicolon();

        cond = self.parse_statement_or_semicolon();

        self.expect_semicolon();

        let st_or_bracket = self.lexer.peek_token();
        if let Token::Special(sp, ..) = st_or_bracket.clone() {
            if sp != ")" {
                action = Some(Box::new(self.parse_statement()));
            } else if sp == ";" {
                self.lexer.next_token();
            }
        } else {
            action = Some(Box::new(self.parse_statement()));
        }

        let must_be_bracket = self.lexer.next_token();

        if let Token::Special(sp, ..) = must_be_bracket.clone() {
            if sp != ")" {
                panic!(
                    "Expected close bracket but got {:?}",
                    must_be_bracket.clone()
                );
            }
        } else {
            panic!(
                "Expected close bracket but got {:?}",
                must_be_bracket.clone()
            );
        }

        let next = self.lexer.peek_token();

        if let Token::Special(sp, ..) = next.clone() {
            if sp == "{" {
                body = self.parse_block();
            } else {
                body = vec![self.parse_statement()];
            }
        } else {
            body = vec![self.parse_statement()];
        }

        return Statement::ForStatement(init, cond, action, body);
    }

    fn parse_statement_or_semicolon(&mut self) -> Option<Box<Statement>> {
        let st_or_semicolon = self.lexer.peek_token();
        if let Token::Special(sp, ..) = st_or_semicolon.clone() {
            if sp != ";" {
                return Some(Box::new(self.parse_statement()));
            }
        } else {
            return Some(Box::new(self.parse_statement()));
        }

        return None;
    }

    fn expect_semicolon(&mut self) {
        let must_be_semicolon = self.lexer.next_token();

        if let Token::Special(sp, ..) = must_be_semicolon.clone() {
            if sp != ";" {
                panic!("Expected semicolon but got {:?}", must_be_semicolon.clone());
            }
        } else {
            panic!("Expected semicolon but got {:?}", must_be_semicolon.clone());
        }
    }
    pub fn parse_statement(&mut self) -> Statement {
        let tok = self.lexer.peek_token();

        match tok.clone() {
            Token::Keyword(keyword, ..) => match &*keyword {
                "let" => self.parse_local(),
                "const" => self.parse_constant(),
                "fn" => self.parse_function(),
                "return" => self.parse_return_statement(),
                "import" => self.parse_import_statement(),
                "use" => self.parse_use_statement(),
                "for" => self.parse_for_statement(),
                _ => panic!("Undefined keyword"),
            },
            Token::Operator(..) => Statement::ExpressionStatement(self.parse_expression()),
            Token::Integer(..) => Statement::ExpressionStatement(self.parse_expression()),
            Token::String(..) => Statement::ExpressionStatement(self.parse_expression()),
            Token::Float(..) => Statement::ExpressionStatement(self.parse_expression()),
            Token::Id(..) => Statement::ExpressionStatement(self.parse_expression()),
            Token::Special(sp, ..) => {
                if sp == "(" {
                    Statement::ExpressionStatement(self.parse_expression())
                } else if sp == "@" {
                    // we have decorators
                    self.parse_decorated_statement()
                } else if sp == ";" {
                    Statement::EmptyStatement
                } else {
                    panic!("Unexpected token {:?}", tok.clone());
                }
            }
            Token::EOF => Statement::EmptyStatement,
            Token::Unknown(_u, ..) => {
                panic!("Unknown token {:?} while parsing statement", tok.clone())
            }
            _ => unreachable!(),
        }
    }

    pub fn parse_block(&mut self) -> Vec<Statement> {
        let mut body = vec![];

        self.lexer.next_token(); // open bracket

        loop {
            let next = self.lexer.peek_token();

            if let Token::Special(sp, ..) = next.clone() {
                if sp == "}" {
                    self.lexer.next_token();
                    break;
                } else if sp == ";" {
                    self.lexer.next_token();
                    continue;
                } else {
                    panic!("Expected close bracket but got {:?}", next.clone());
                }
            }
            body.push(self.parse_statement());
        }

        return body;
    }

    pub fn parse_param(&mut self) -> Param {
        let next = self.lexer.peek_token();

        if let Token::Id(name, ..) = next.clone() {
            self.lexer.next_token();

            let mut _type = None;
            let peek = self.lexer.peek_token();

            if let Token::Special(sp, ..) = peek {
                if sp == ":" {
                    _type = Some(self.parse_colon_type());
                }
            }

            return Param { name, _type };
        } else {
            panic!("Expected identifier but got {:?}", next.clone());
        }
    }
    pub fn parse_params(&mut self) -> Vec<Param> {
        let mut params = vec![];

        self.lexer.next_token(); // open bracket

        loop {
            let next = self.lexer.peek_token();

            if let Token::Id(name, ..) = next.clone() {
                self.lexer.next_token();

                let mut _type = None;
                let peek = self.lexer.peek_token();

                if let Token::Special(sp, ..) = peek {
                    if sp == ":" {
                        _type = Some(self.parse_colon_type());
                    }
                }

                params.push(Param { name, _type });
            } else if let Token::Special(sp, ..) = next.clone() {
                if sp == ")" {
                    self.lexer.next_token();
                    break;
                } else if sp == "," {
                    self.lexer.next_token();
                } else {
                    panic!("Expected close bracket but got {:?}", next.clone());
                }
            } else {
                panic!("Unexpected token {:?}", next.clone());
            }
        }
        return params;
    }

    pub fn parse_arguments(&mut self) -> Vec<Expression> {
        let mut arguments = vec![];

        self.lexer.next_token(); // open bracket

        loop {
            let next = self.lexer.peek_token();
            if let Token::Special(sp, ..) = next.clone() {
                if sp == ")" {
                    self.lexer.next_token();
                    break;
                } else if sp == "(" {
                    arguments.push(self.parse_expression());
                } else if sp == "," {
                    self.lexer.next_token();
                } else {
                    panic!("Expected close bracket but got {:?}", next.clone());
                }
            } else {
                arguments.push(self.parse_expression());
            }
        }
        return arguments;
    }

    pub fn parse_colon_type(&mut self) -> String {
        self.lexer.next_token(); // colon

        let type_tok = self.lexer.next_token();

        if let Token::Id(name, ..) = type_tok {
            return name.clone();
        } else {
            panic!("Expected type name but got {:?}", type_tok.clone());
        }
    }

    pub fn parse_function(&mut self) -> Statement {
        self.lexer.next_token(); // fn

        let id_tok = self.lexer.next_token();

        let id = if let Token::Id(_id, ..) = id_tok {
            _id
        } else {
            panic!("Expected identifier but got {:?}", id_tok);
        };

        let next = self.lexer.peek_token();

        let mut params = vec![];

        let mut body = vec![];

        let mut return_type = None;

        match next.clone() {
            Token::Special(sp, _) => {
                let special_tok = self.lexer.peek_token();

                if sp == ":" {
                    // we have parenthless function declaration
                    return_type = Some(self.parse_colon_type());

                    let must_be_bracket = self.lexer.peek_token();

                    if let Token::Special(sp, ..) = must_be_bracket.clone() {
                        if sp != "{" {
                            panic!("Expected {} but got {:?}", "{", must_be_bracket.clone());
                        }

                        body = self.parse_block();
                    } else {
                        panic!("Expected {} but got {:?}", "{", must_be_bracket.clone());
                    }
                } else if sp == "(" {
                    // we have parenth function declration
                    params = self.parse_params();

                    let next = self.lexer.peek_token();

                    if let Token::Special(sp, ..) = next.clone() {
                        if sp == ":" {
                            return_type = Some(self.parse_colon_type());

                            let must_be_bracket = self.lexer.peek_token();

                            if let Token::Special(sp, ..) = must_be_bracket.clone() {
                                if sp != "{" {
                                    panic!(
                                        "Expected {} but got {:?}",
                                        "{",
                                        must_be_bracket.clone()
                                    );
                                }

                                body = self.parse_block();
                            } else {
                                panic!("Expected {} but got {:?}", "{", must_be_bracket.clone());
                            }
                        } else if sp == "{" {
                            body = self.parse_block();
                        } else {
                            panic!("Expected open bracket or colon but got {:?}", next.clone());
                        }
                    } else {
                        panic!("Expected {} but got {:?}", "{", next.clone());
                    }
                } else if sp == "{" {
                    // we have parenthless and return type less function declration
                    body = self.parse_block();
                } else {
                    panic!(
                        "Expected colon or open bracket but got {:?}",
                        special_tok.clone()
                    );
                }
            }
            _ => return_type = None,
        }

        return Statement::FunctionDeclarationStatement(id.clone(), params, return_type, body);
    }

    pub fn parse_constant(&mut self) -> Statement {
        self.lexer.next_token(); // const
        let id_tok = self.lexer.next_token();

        let id = if let Token::Id(_id, ..) = id_tok {
            _id
        } else {
            panic!("Expected identifier but got {:?}", id_tok);
        };

        let mut _type: Option<String> = None;
        let expr: Expression;

        let next = self.lexer.peek_token();

        if let Token::Special(sp, ..) = next {
            if sp == ":" {
                _type = Some(self.parse_colon_type());
            }
        }

        let next = self.lexer.peek_token();

        if let Token::Operator(op, ..) = next.clone() {
            if op == "=" {
                self.lexer.next_token();

                expr = self.parse_expression();
            } else {
                panic!("Expected \"=\" unexpected operator {:?}", next.clone());
            }
        } else {
            panic!("Expected \"=\" unexpected token {:?}", next.clone());
        }

        return Statement::ConstantStatement(id, _type, expr);
    }

    pub fn parse_local(&mut self) -> Statement {
        self.lexer.next_token(); // let
        let id_tok = self.lexer.next_token();

        let id = if let Token::Id(_id, ..) = id_tok {
            _id
        } else {
            panic!("Expected identifier but got {:?}", id_tok);
        };

        let mut _type: Option<String> = None;
        let mut expr: Option<Expression> = None;

        let next = self.lexer.peek_token();
        match next.clone() {
            Token::Operator(_op, _) => {
                let equal_tok = self.lexer.next_token();

                if _op != "=" {
                    panic!(
                        "Expected assignment operator but got {:?}",
                        equal_tok.clone()
                    );
                }

                expr = Some(self.parse_expression());

                _type = None;
            }
            Token::Special(sp, _) => {
                if sp == ":" {
                    _type = Some(self.parse_colon_type());
                }
            }
            _ => _type = None,
        }

        if _type != None {
            let next = self.lexer.peek_token();
            match next.clone() {
                Token::Operator(_op, _) => {
                    let equal_tok = self.lexer.next_token();

                    if _op != "=" {
                        panic!(
                            "Expected assignment operator but got {:?}",
                            equal_tok.clone()
                        );
                    }

                    expr = Some(self.parse_expression());
                }
                _ => expr = None,
            }
        }

        return Statement::LocalStatement(id, _type, expr);
    }

    pub fn parse_expression(&mut self) -> Expression {
        return self.parse_expression_recursive();
    }

    pub fn parse_assign_expression(&mut self) -> Expression {
        let tok = self.lexer.peek_token();

        if let Token::Special(sp, ..) = tok.clone() {
            if sp == "(" {
                self.lexer.next_token();

                let expression = self.parse_expression_recursive();

                let must_be_bracket = self.lexer.next_token();

                if let Token::Special(_sp, ..) = must_be_bracket.clone() {
                    if _sp != ")" {
                        panic!(
                            "Expected close bracket but got {:?}",
                            must_be_bracket.clone()
                        );
                    }
                } else {
                    panic!(
                        "Expected close bracket but got {:?}",
                        must_be_bracket.clone()
                    );
                }

                return expression;
            }
        } else if let Token::Operator(op, ..) = tok.clone() {
            if self.unary_operators.contains(&op) {
                self.lexer.next_token();

                return Expression::PreUnaryExpression(
                    tok.clone(),
                    Box::new(self.parse_expression()),
                );
            } else {
                panic!("Expected unary operator {:?}", tok.clone());
            }
        } else if let Token::Id(id, ..) = tok.clone() {
            self.lexer.next_token();

            return Expression::IdLiteral(id.clone());
        } else if let Token::String(str, ..) = tok.clone() {
            self.lexer.next_token();

            return Expression::StringLiteral(str.clone());
        } else if let Token::Integer(num, ..) = tok.clone() {
            self.lexer.next_token();

            return Expression::IntegerLiteral(num);
        } else if let Token::Float(num, ..) = tok.clone() {
            self.lexer.next_token();

            return Expression::FloatLiteral(num);
        } else if let Token::Keyword(kw, ..) = tok.clone() {
            if kw == "fn" {
                return self.parse_fn_expression();
            } else {
                panic!("Unexpected keyword {:?}", tok.clone());
            }
        }

        panic!("Error while parsing expression {:?}", tok.clone());
    }

    pub fn parse_fn_expression(&mut self) -> Expression {
        let mut params = vec![];
        let mut statements = vec![];
        let mut expr = None;
        let mut return_type = None;

        self.lexer.next_token();

        let next = self.lexer.peek_token();

        if let Token::Special(sp, ..) = next.clone() {
            if sp == "(" {
                params = self.parse_params();
            } else {
                panic!("Expected id or open bracket but got {:?}", next.clone());
            }
        } else if let Token::Id(id, ..) = next.clone() {
            let param = self.parse_param();

            params = vec![param];
        }

        let next = self.lexer.peek_token();

        if let Token::Special(sp, ..) = next.clone() {
            if sp == ":" {
                return_type = Some(self.parse_colon_type());
            }
        }
        let next = self.lexer.next_token();

        if let Token::Special(sp, ..) = next.clone() {
            if sp != "=>" {
                panic!("Expected \"=>\" but got {:?}", next.clone());
            } else {
            }
        } else {
            panic!("Expected \"=>\" but got {:?}", next.clone());
        }

        let next = self.lexer.peek_token();

        if let Token::Special(sp, ..) = next.clone() {
            if sp == "{" {
                statements = self.parse_block();
            } else {
                expr = Some(Box::new(self.parse_expression()));
            }
        } else {
            expr = Some(Box::new(self.parse_expression()));
        }

        return Expression::FunctionExpression(params, return_type, expr, statements);
    }

    pub fn parse_dot_expression(&mut self) -> Expression {
        let mut expression = self.parse_assign_expression();

        if let Token::Operator(op, ..) = self.lexer.peek_token() {
            if op == "="
                || op == "*="
                || op == "/="
                || op == "%="
                || op == "+="
                || op == "-="
                || op == "**="
            {
                let tok = self.lexer.next_token();

                let rhs = self.parse_dot_expression();
                let lhs = expression;

                expression = Expression::AssignmentExpression(tok, Box::new(lhs), Box::new(rhs));
            } else if self.unary_operators.contains(&op) {
                let state = self.lexer.save_state();

                let tok = self.lexer.next_token();

                let next = self.lexer.peek_token();

                if let Token::Id(..) = next {
                    self.lexer.load_state(state);
                    return expression;
                } else if let Token::String(..) = next {
                    self.lexer.load_state(state);
                    return expression;
                } else if let Token::Float(..) = next {
                    self.lexer.load_state(state);
                    return expression;
                } else if let Token::Integer(..) = next {
                    self.lexer.load_state(state);
                    return expression;
                }

                expression = Expression::PostUnaryExpression(tok, Box::new(expression));
            }
        }

        return expression;
    }
    pub fn parse_call_expression(&mut self) -> Expression {
        let mut expression = self.parse_pow_expression();

        loop {
            let token = self.lexer.peek_token();
            if let Token::Special(sp, ..) = token.clone() {
                if sp == "(" {
                    let args = self.parse_arguments();

                    expression = Expression::CallExpression(Box::new(expression), args);
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        return expression;
    }

    pub fn parse_pow_expression(&mut self) -> Expression {
        let mut expression = self.parse_dot_expression();

        let token = self.lexer.peek_token();
        if let Token::Operator(op, ..) = token {
            if op == "." {
                let tok = self.lexer.next_token();

                let rhs = self.parse_pow_expression();
                let lhs = expression;

                expression = Expression::DotExpression(Box::new(lhs), Box::new(rhs));
            }
        }

        return expression;
    }

    pub fn parse_factor_expression(&mut self) -> Expression {
        let mut expression = self.parse_call_expression();

        let token = self.lexer.peek_token();
        if let Token::Operator(op, ..) = token {
            if op == "**" {
                let tok = self.lexer.next_token();

                let rhs = self.parse_factor_expression();
                let lhs = expression;

                expression = Expression::BinaryExpression(tok, Box::new(lhs), Box::new(rhs));
            }
        }

        return expression;
    }

    pub fn parse_term_expression(&mut self) -> Expression {
        let mut expression = self.parse_factor_expression();

        if let Token::Operator(op, ..) = self.lexer.peek_token() {
            if op == "*" || op == "/" || op == "%" {
                let tok = self.lexer.next_token();

                let rhs = self.parse_term_expression();
                let lhs = expression;

                expression = Expression::BinaryExpression(tok, Box::new(lhs), Box::new(rhs));
            }
        }

        return expression;
    }

    pub fn parse_expression_recursive(&mut self) -> Expression {
        let mut expression = self.parse_term_expression();

        let token = self.lexer.peek_token();

        if let Token::Operator(op, ..) = token {
            if op == "+"
                || op == "-"
                || op == "<"
                || op == ">"
                || op == ">="
                || op == "<="
                || op == "=="
                || op == "!="
            {
                let tok = self.lexer.next_token();

                let rhs = self.parse_expression_recursive();
                let lhs = expression;

                expression = Expression::BinaryExpression(tok, Box::new(lhs), Box::new(rhs));
            }
        }

        return expression;
    }
    pub fn parse_expression_shunting_yard(&mut self) -> Expression {
        #[derive(Clone)]
        struct StackElement {
            pub token: Token,
            pub is_unary: bool,
            pub is_function: bool,
            pub function_args_count: u32,
        }

        let mut output_tokens: Vec<StackElement> = vec![];
        let mut stack: Vec<StackElement> = vec![];
        let mut value_stack = vec![];
        let mut args_count_stack = vec![];

        let mut prev: Option<Token> = None;
        let mut last_function: Option<&mut StackElement> = None;

        loop {
            let symbol = self.lexer.peek_token();
            match symbol.clone() {
                Token::Integer(_, _) => {
                    self.lexer.next_token();

                    output_tokens.push(StackElement {
                        token: symbol.clone(),
                        is_unary: false,
                        is_function: false,
                        function_args_count: 0,
                    });

                    if value_stack.len() != 0 {
                        value_stack.pop();
                        value_stack.push(true);
                    }
                }
                Token::Float(_, _) => {
                    self.lexer.next_token();
                    output_tokens.push(StackElement {
                        token: symbol.clone(),
                        is_unary: false,
                        is_function: false,
                        function_args_count: 0,
                    });
                    if value_stack.len() != 0 {
                        value_stack.pop();
                        value_stack.push(true);
                    }
                }
                Token::String(_, _) => {
                    self.lexer.next_token();
                    output_tokens.push(StackElement {
                        token: symbol.clone(),
                        is_unary: false,
                        is_function: false,
                        function_args_count: 0,
                    });
                    if value_stack.len() != 0 {
                        value_stack.pop();
                        value_stack.push(true);
                    }
                }
                Token::Id(_, _) => {
                    self.lexer.next_token();

                    let mut is_function = false;

                    if value_stack.len() != 0 {
                        value_stack.pop();
                        value_stack.push(true);
                    }

                    if let Token::String(sp, ..) = self.lexer.peek_token().clone() {
                        if sp == "(" {
                            args_count_stack.push(0);
                            is_function = true;
                            value_stack.push(false);
                        }
                    }
                    output_tokens.push(StackElement {
                        token: symbol.clone(),
                        is_unary: false,
                        is_function,
                        function_args_count: 0,
                    });
                }
                Token::Operator(_op, _) => {
                    self.lexer.next_token();
                    let priority = self.operators.get(&_op).expect("");

                    if let Some(Token::Operator(_op, ..)) = prev {
                        stack.push(StackElement {
                            token: symbol.clone(),
                            is_unary: true,
                            is_function: false,
                            function_args_count: 0,
                        });
                    } else if prev == None {
                        stack.push(StackElement {
                            token: symbol.clone(),
                            is_unary: true,
                            is_function: false,
                            function_args_count: 0,
                        });
                    } else {
                        if let Some(StackElement {
                            token: Token::Operator(_op, ..),
                            ..
                        }) = stack.last()
                        {
                            if priority <= self.operators.get(&_op.clone()).expect("") {
                                output_tokens.push(stack.pop().unwrap());
                            }
                        }

                        stack.push(StackElement {
                            token: symbol.clone(),
                            is_unary: false,
                            is_function: false,
                            function_args_count: 0,
                        });
                    }
                }
                Token::Special(ch, _) => {
                    if ch == "(" {
                        self.lexer.next_token();

                        stack.push(StackElement {
                            token: symbol.clone(),
                            is_unary: false,
                            is_function: false,
                            function_args_count: 0,
                        });
                    } else if ch == ")" {
                        self.lexer.next_token();

                        loop {
                            let tok = stack.pop().expect("Invalid format");

                            if let Token::Special(_, ..) = tok.clone().token {
                                break;
                            } else {
                                output_tokens.push(tok.clone());
                            }
                        }

                        let last = stack.last();

                        if let Some(el) = last {
                            if el.is_function {
                                let mut f = stack.pop().unwrap();
                                let mut a = args_count_stack.pop().unwrap();
                                let w = value_stack.pop().unwrap();

                                if w {
                                    a += 1;
                                }

                                f.function_args_count = a;

                                output_tokens.push(f);
                            }
                        }
                    } else if ch == "," {
                        loop {
                            let tok = stack.last().expect("Invalid format");

                            if let Token::Special(_, ..) = tok.clone().token {
                                break;
                            } else {
                                output_tokens.push(tok.clone());
                                stack.pop();
                            }
                        }

                        let value = value_stack.pop().unwrap();

                        if value {
                            let count = args_count_stack.pop().unwrap();
                            args_count_stack.push(count + 1);
                        }

                        value_stack.push(false);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
            prev = Some(symbol.clone());
        }

        while stack.len() != 0 {
            output_tokens.push(stack.pop().unwrap());
        }

        let mut stack: Vec<Expression> = vec![];

        for tok in output_tokens {
            match tok.token.clone() {
                Token::Integer(num, _) => {
                    stack.push(Expression::IntegerLiteral(num));
                }
                Token::Float(num, _) => {
                    stack.push(Expression::FloatLiteral(num));
                }
                Token::Id(id, _) => {
                    if tok.is_function {
                        let mut args = vec![];

                        for i in 0..tok.function_args_count {
                            args.push(stack.pop().unwrap());
                        }

                        stack.push(Expression::CallExpression(
                            Box::new(Expression::IdLiteral(id)),
                            args,
                        ));
                    } else {
                        stack.push(Expression::IdLiteral(id));
                    }
                }
                Token::String(str, _) => {
                    stack.push(Expression::StringLiteral(str));
                }
                Token::Operator(op, _) => {
                    let s = stack.pop().unwrap();
                    if !tok.is_unary {
                        let f = stack.pop().unwrap();

                        stack.push(Expression::BinaryExpression(
                            tok.token.clone(),
                            Box::new(f),
                            Box::new(s),
                        ));
                    } else {
                        stack.push(Expression::PreUnaryExpression(
                            tok.token.clone(),
                            Box::new(s),
                        ));
                    }
                }
                _ => unreachable!(),
            }
        }

        return stack.pop().unwrap();
    }

    pub fn parse_return_statement(&mut self) -> Statement {
        self.lexer.next_token(); // return

        let next = self.lexer.peek_token();

        let mut expr = None;

        if let Token::Special(sp, ..) = next {
            if sp == "(" {
                self.lexer.next_token();
                expr = Some(self.parse_expression());
            }
        } else if let Token::Id(..) = next {
            expr = Some(self.parse_expression());
        } else if let Token::Integer(..) = next {
            expr = Some(self.parse_expression());
        } else if let Token::String(..) = next {
            expr = Some(self.parse_expression());
        } else if let Token::Float(..) = next {
            expr = Some(self.parse_expression());
        } else if let Token::Keyword(kw, ..) = next {
            if kw == "fn" {
                expr = Some(self.parse_expression());
            }
        }

        return Statement::ReturnStatement(expr);
    }
}
