use crate::compiler::context::Context;
use derivative::Derivative;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Derivative)]
#[derivative(Debug, Clone)]
pub struct Import {
    pub(crate) name: String, // relative name
    pub(crate) context: Rc<RefCell<Context>>,
    pub(crate) path: String, // absolute path to module
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ImportTable {
    pub(crate) imports: Vec<Import>,
}

impl ImportTable {
    pub(crate) fn add(&mut self, import: Import) -> Result<(), &str> {
        let import_name = import.name.clone();

        for import in &self.imports {
            if import.name == import_name {
                return Err("The same name already imported");
            }
        }

        self.imports.push(import);

        Ok(())
    }
}
