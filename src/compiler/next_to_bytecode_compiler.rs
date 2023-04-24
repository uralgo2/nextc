use crate::compiler::context::Context;
use crate::compiler::functions::FunctionTable;
use crate::compiler::info_collector_visitor::InfoCollectorVisitor;
use crate::compiler::types::{Type, TypeTable};
use crate::compiler::visitor::Visitor;
use crate::parser::node::Statement;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct NextToByteCodeCompiler {
    pub context: Vec<Rc<RefCell<Context>>>,
    pub types: TypeTable,
    pub globals: HashMap<String, Rc<RefCell<Type>>>,
    pub top_level_functions: FunctionTable,
}

impl NextToByteCodeCompiler {
    pub fn compile_program(&mut self, program: Vec<Statement>) -> Rc<RefCell<Context>> {
        let mut info_collector_visitor = InfoCollectorVisitor::new();
        info_collector_visitor.visit_program(program.clone());
        let info_ctx = info_collector_visitor.get_top_level_ctx();

        //let compiler_visitor = CompilerVisitor::new(Rc::new(info_ctx));

        //compiler_visitor.visit_program(program.clone());

        //let compiled_ctx = compiler_visitor.get_top_level_ctx();

        return info_ctx; //compiled_ctx;
    }
}
