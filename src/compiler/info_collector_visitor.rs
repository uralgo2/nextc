use crate::compiler::context::Context;
use crate::compiler::exports::ExportTable;
use crate::compiler::functions::{Function, FunctionParam, FunctionTable};
use crate::compiler::imports::ImportTable;
use crate::compiler::types::{Type, TypeTable};
use crate::compiler::uses::UsesTable;
use crate::compiler::visitor::Visitor;
use crate::parser::node::{Expression, Statement};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct InfoCollectorVisitor {
    context: Vec<Rc<RefCell<Context>>>,
}

impl InfoCollectorVisitor {
    pub fn new() -> Self {
        InfoCollectorVisitor { context: vec![] }
    }

    pub fn push_ctx(&mut self, ctx: Rc<RefCell<Context>>) {
        self.context.push(Rc::clone(&ctx));
    }

    pub fn pop_ctx(&mut self) -> Rc<RefCell<Context>> {
        return self.context.pop().unwrap();
    }

    pub fn get_current_ctx(&self) -> Rc<RefCell<Context>> {
        return Rc::clone(self.context.last().unwrap());
    }

    pub fn get_top_level_ctx(&mut self) -> Rc<RefCell<Context>> {
        return self.pop_ctx();
    }
}

impl Visitor<()> for InfoCollectorVisitor {
    fn visit_program(&mut self, program: Vec<Statement>) -> () {
        self.push_ctx(Rc::new(RefCell::new(Context {
            locals: HashMap::new(),
            functions: FunctionTable { functions: vec![] },
            types: TypeTable {
                types: vec![
                    Rc::new(RefCell::new(Type {
                        name: "unknown".to_string(),
                        methods: FunctionTable { functions: vec![] },
                        virtual_methods: FunctionTable { functions: vec![] },
                        constructors: FunctionTable { functions: vec![] },
                        static_methods: FunctionTable { functions: vec![] },
                        getters: FunctionTable { functions: vec![] },
                        setters: FunctionTable { functions: vec![] },
                        fields: vec![],
                        static_fields: vec![],
                        parent: None,
                    })),
                    Rc::new(RefCell::new(Type {
                        name: "int".to_string(),
                        methods: FunctionTable { functions: vec![] },
                        virtual_methods: FunctionTable { functions: vec![] },
                        constructors: FunctionTable { functions: vec![] },
                        static_methods: FunctionTable { functions: vec![] },
                        getters: FunctionTable { functions: vec![] },
                        setters: FunctionTable { functions: vec![] },
                        fields: vec![],
                        static_fields: vec![],
                        parent: None,
                    })),
                    Rc::new(RefCell::new(Type {
                        name: "long".to_string(),
                        methods: FunctionTable { functions: vec![] },
                        virtual_methods: FunctionTable { functions: vec![] },
                        constructors: FunctionTable { functions: vec![] },
                        static_methods: FunctionTable { functions: vec![] },
                        getters: FunctionTable { functions: vec![] },
                        setters: FunctionTable { functions: vec![] },
                        fields: vec![],
                        static_fields: vec![],
                        parent: None,
                    })),
                    Rc::new(RefCell::new(Type {
                        name: "byte".to_string(),
                        methods: FunctionTable { functions: vec![] },
                        virtual_methods: FunctionTable { functions: vec![] },
                        constructors: FunctionTable { functions: vec![] },
                        static_methods: FunctionTable { functions: vec![] },
                        getters: FunctionTable { functions: vec![] },
                        setters: FunctionTable { functions: vec![] },
                        fields: vec![],
                        static_fields: vec![],
                        parent: None,
                    })),
                ],
            },
            parent: None,
            imports: ImportTable { imports: vec![] },
            exports: ExportTable { exports: vec![] },
            uses: UsesTable { uses: vec![] },
        })));

        for statement in program {
            self.visit_statement(statement);
        }
    }

    fn visit_statement(&mut self, statement: Statement) -> () {
        match statement {
            Statement::ExpressionStatement(_) => self.visit_expression_statement(statement),
            Statement::LocalStatement(_, _, _) => self.visit_local_statement(statement),
            Statement::ConstantStatement(_, _, _) => self.visit_constant_statement(statement),
            Statement::FunctionDeclarationStatement(_, _, _, _) => {
                self.visit_function_declaration_statement(statement)
            }
            Statement::ReturnStatement(_) => self.visit_return_statement(statement),
            Statement::ImportStatement(_) => self.visit_import_statement(statement),
            Statement::UseStatement(_) => self.visit_use_statement(statement),
            Statement::DecoratedStatement(_, _) => self.visit_decorated_statement(statement),
            Statement::IfStatement(_, _, _) => self.visit_if_statement(statement),
            Statement::ForStatement(_, _, _, _) => self.visit_for_statement(statement),
            Statement::While(_, _) => self.visit_while_statement(statement),
            Statement::EmptyStatement => self.visit_empty_statement(statement),
        }
    }

    fn visit_expression_statement(&mut self, statement: Statement) -> () {
        ()
    }

    fn visit_local_statement(&mut self, statement: Statement) -> () {
        ()
    }

    fn visit_constant_statement(&mut self, statement: Statement) -> () {
        ()
    }

    fn visit_function_declaration_statement(&mut self, statement: Statement) -> () {
        let mut ctx = Rc::clone(&self.get_current_ctx());

        let Statement::FunctionDeclarationStatement(name, params, return_type, body) = statement
            else {unreachable!()};

        let mut locals = HashMap::new();
        let mut fun_params = vec![];

        for param in params {
            let _type = ctx.borrow().get_type_or_unknown(param._type);

            locals.insert(param.name.clone(), _type.clone());
            fun_params.push(FunctionParam {
                name: param.name.clone(),
                _type,
            });
        }

        let mut fun_ctx = Rc::new(RefCell::new(Context {
            locals,
            functions: FunctionTable { functions: vec![] },
            types: TypeTable { types: vec![] },
            parent: Some(Rc::clone(&ctx)),
            imports: ImportTable { imports: vec![] },
            exports: ExportTable { exports: vec![] },
            uses: UsesTable { uses: vec![] },
        }));

        let mut fun = Function {
            name: name.clone(),
            return_type: ctx.borrow().get_type_or_unknown(return_type),
            params: fun_params,
            bytecode: vec![],
            mangled_name: name.clone(),
            ctx: Rc::clone(&fun_ctx),
        };

        ctx.borrow_mut()
            .functions
            .add(fun)
            .expect("Function with the same signature already exist");

        self.push_ctx(fun_ctx);

        for statement in body {
            self.visit_statement(statement);
        }

        let fun_ctx = self.pop_ctx();

        ()
    }

    fn visit_return_statement(&mut self, statement: Statement) -> () {
        ()
    }

    fn visit_import_statement(&mut self, statement: Statement) -> () {
        let mut ctx = Rc::clone(&self.get_current_ctx());

        let Statement::ImportStatement(import_name) = statement
            else {unreachable!()};

        ctx.borrow_mut()
            .imports
            .add(import_name)
            .expect("Name already imported");

        ()
    }

    fn visit_use_statement(&mut self, statement: Statement) -> () {
        ()
    }

    fn visit_decorated_statement(&mut self, statement: Statement) -> () {
        ()
    }

    fn visit_if_statement(&mut self, statement: Statement) -> () {
        ()
    }

    fn visit_for_statement(&mut self, statement: Statement) -> () {
        ()
    }

    fn visit_while_statement(&mut self, statement: Statement) -> () {
        ()
    }

    fn visit_empty_statement(&mut self, statement: Statement) -> () {
        ()
    }

    fn visit_binary_expression(&mut self, expression: Expression) -> () {
        ()
    }

    fn visit_assignment_expression(&mut self, expression: Expression) -> () {
        ()
    }

    fn visit_pre_unary_expression(&mut self, expression: Expression) -> () {
        ()
    }

    fn visit_post_unary_expression(&mut self, expression: Expression) -> () {
        ()
    }

    fn visit_dot_expression(&mut self, expression: Expression) -> () {
        ()
    }

    fn visit_call_expression(&mut self, expression: Expression) -> () {
        ()
    }

    fn visit_function_expression(&mut self, expression: Expression) -> () {
        ()
    }

    fn visit_integer_literal_expression(&mut self, expression: Expression) -> () {
        ()
    }

    fn visit_float_literal_expression(&mut self, expression: Expression) -> () {
        ()
    }

    fn visit_string_literal_expression(&mut self, expression: Expression) -> () {
        ()
    }

    fn visit_id_literal_expression(&mut self, expression: Expression) -> () {
        ()
    }
}
