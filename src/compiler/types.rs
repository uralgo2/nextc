use crate::compiler::functions::FunctionTable;
use derivative::Derivative;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Derivative)]
#[derivative(PartialEq, Clone, Debug)]
pub struct TypeField {
    pub name: String,
    pub _type: Rc<RefCell<Type>>,
}

#[derive(Derivative)]
#[derivative(PartialEq, Debug)]
pub struct Type {
    pub name: String,

    #[derivative(Debug = "ignore")]
    pub methods: FunctionTable,
    #[derivative(Debug = "ignore")]
    pub virtual_methods: FunctionTable,
    #[derivative(Debug = "ignore")]
    pub constructors: FunctionTable,
    #[derivative(Debug = "ignore")]
    pub static_methods: FunctionTable,
    #[derivative(Debug = "ignore")]
    pub getters: FunctionTable,
    #[derivative(Debug = "ignore")]
    pub setters: FunctionTable,

    #[derivative(Debug = "ignore")]
    pub fields: Vec<TypeField>,
    #[derivative(Debug = "ignore")]
    pub static_fields: Vec<TypeField>,

    pub parent: Option<Rc<RefCell<Type>>>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct TypeTable {
    pub types: Vec<Rc<RefCell<Type>>>,
}
impl TypeTable {
    pub fn get_by_name(&self, name: &str) -> Option<Rc<RefCell<Type>>> {
        for _type in &self.types {
            let moved = Rc::clone(_type);

            if moved.borrow().name == name {
                return Some(moved);
            }
        }

        return None;
    }

    pub fn add(&mut self, _type: Type) -> Result<(), &str> {
        if let Some(..) = self.get_by_name(_type.name.as_str()) {
            return Err("Type with same name already declarated");
        }

        self.types.push(Rc::new(RefCell::new(_type)));

        return Ok(());
    }
}
