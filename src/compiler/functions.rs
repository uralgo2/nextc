use crate::compiler::context::Context;
use crate::compiler::types::Type;
use crate::core::opcodes::OpCode;
use derivative::Derivative;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Derivative)]
#[derivative(Clone, Debug)]
pub struct FunctionParam {
    pub name: String,
    pub _type: Rc<RefCell<Type>>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Function {
    pub name: String,
    pub return_type: Rc<RefCell<Type>>,
    pub params: Vec<FunctionParam>,

    pub bytecode: Vec<OpCode>,
    pub mangled_name: String,

    pub ctx: Rc<RefCell<Context>>,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct FunctionTable {
    pub functions: Vec<Rc<RefCell<Function>>>,
}

impl Display for Function {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Function id=\"{}\" return_type=\"{}\" params={:?}",
            self.name,
            self.return_type.borrow().name,
            self.params
        )
    }
}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }

        /*
        if self.return_type != other.return_type { // overloading by return type is not a cool :(
            return false;
        }
        */

        if self.params.len() != other.params.len() {
            return false;
        }

        for i in 0..self.params.len() {
            if self.params.get(i).unwrap()._type != other.params.get(i).unwrap()._type {
                return false;
            }
        }

        return true;
    }
}

impl PartialEq for FunctionTable {
    fn eq(&self, other: &Self) -> bool {
        if self.functions.len() != other.functions.len() {
            return false;
        }

        for i in 0..self.functions.len() {
            if *self.functions.get(i).unwrap() != *other.functions.get(i).unwrap() {
                return false;
            }
        }

        return true;
    }
}

impl FunctionTable {
    pub fn get_by_name(&self, name: &str) -> Option<Vec<Rc<RefCell<Function>>>> {
        let mut functions = vec![];

        for function in &self.functions {
            if function.borrow().name == name {
                functions.push(Rc::clone(function));
            }
        }

        if functions.is_empty() {
            return None;
        }

        return Some(functions);
    }

    pub fn get_first_by_name(&self, name: &str) -> Option<Rc<RefCell<Function>>> {
        for function in &self.functions {
            if function.borrow().name == name {
                return Some(Rc::clone(function));
            }
        }

        return None;
    }

    pub fn get_by_signature(
        &self,
        name: &str,
        params_types: Vec<Rc<RefCell<Type>>>,
    ) -> Option<Rc<RefCell<Function>>> {
        let candidates = self.get_by_name(name);

        if let Some(vector) = candidates {
            'outer: for fun in vector {
                if fun.borrow().params.len() != params_types.len() {
                    continue;
                }

                for i in 0..params_types.len() {
                    if fun.borrow().params.get(i).unwrap()._type != *params_types.get(i).unwrap() {
                        continue 'outer;
                    }
                }
                return Some(Rc::clone(&fun));
            }
        }

        return None;
    }

    pub fn add(&mut self, fun: Function) -> Result<(), &str> {
        let params_types = fun.params.iter().map(|param| param._type.clone()).collect();

        if let Some(..) = self.get_by_signature(fun.name.as_str(), params_types) {
            return Err("Function with the same signature already declarated");
        }

        self.functions.push(Rc::new(RefCell::new(fun)));

        return Ok(());
    }
}
