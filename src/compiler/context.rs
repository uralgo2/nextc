use crate::compiler::exports::ExportTable;
use crate::compiler::functions::FunctionTable;
use crate::compiler::imports::ImportTable;
use crate::compiler::types::{Type, TypeTable};
use crate::compiler::uses::UsesTable;
use derivative::Derivative;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Context {
    pub locals: HashMap<String, Rc<RefCell<Type>>>,
    pub functions: FunctionTable,
    pub types: TypeTable,
    pub parent: Option<Rc<RefCell<Context>>>,
    pub imports: ImportTable,
    pub exports: ExportTable,
    pub uses: UsesTable,
}
impl Context {
    pub fn get_type_or_unknown(&self, type_name: Option<String>) -> Rc<RefCell<Type>> {
        let mut found_type = None;

        if let Some(name) = type_name.clone() {
            found_type = self.types.get_by_name(name.as_str());
        } else {
            return self.get_type_or_unknown(Some("unknown".to_string()));
        }

        if let Some(found) = found_type {
            return found;
        }

        if let Some(mut parent_ctx) = self.parent.clone() {
            return parent_ctx.borrow().get_type_or_unknown(type_name.clone());
        }

        panic!("Type with the same name does not exist");
    }
}
