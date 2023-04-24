use crate::parser::node::{Expression, Statement};

pub trait Visitor<T> {
    fn visit_program(&mut self, program: Vec<Statement>) -> T;
    fn visit_statement(&mut self, statement: Statement) -> T;
    fn visit_expression_statement(&mut self, statement: Statement) -> T;
    fn visit_local_statement(&mut self, statement: Statement) -> T;
    fn visit_constant_statement(&mut self, statement: Statement) -> T;
    fn visit_function_declaration_statement(&mut self, statement: Statement) -> T;
    fn visit_return_statement(&mut self, statement: Statement) -> T;
    fn visit_import_statement(&mut self, statement: Statement) -> T;
    fn visit_use_statement(&mut self, statement: Statement) -> T;
    fn visit_decorated_statement(&mut self, statement: Statement) -> T;
    fn visit_if_statement(&mut self, statement: Statement) -> T;
    fn visit_for_statement(&mut self, statement: Statement) -> T;
    fn visit_while_statement(&mut self, statement: Statement) -> T;
    fn visit_empty_statement(&mut self, statement: Statement) -> T;

    fn visit_binary_expression(&mut self, expression: Expression) -> T;
    fn visit_assignment_expression(&mut self, expression: Expression) -> T;
    fn visit_pre_unary_expression(&mut self, expression: Expression) -> T;
    fn visit_post_unary_expression(&mut self, expression: Expression) -> T;
    fn visit_dot_expression(&mut self, expression: Expression) -> T;
    fn visit_call_expression(&mut self, expression: Expression) -> T;
    fn visit_function_expression(&mut self, expression: Expression) -> T;
    fn visit_integer_literal_expression(&mut self, expression: Expression) -> T;
    fn visit_float_literal_expression(&mut self, expression: Expression) -> T;
    fn visit_string_literal_expression(&mut self, expression: Expression) -> T;
    fn visit_id_literal_expression(&mut self, expression: Expression) -> T;
}
