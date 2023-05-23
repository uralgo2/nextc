use crate::compiler::context::Context;
use crate::compiler::exports::ExportTable;
use crate::compiler::functions::FunctionTable;
use crate::compiler::imports::{Import, ImportTable};
use crate::compiler::info_collector_visitor::InfoCollectorVisitor;
use crate::compiler::types::{Type, TypeTable};
use crate::compiler::uses::UsesTable;
use crate::compiler::visitor::Visitor;
use crate::lexer::Lexer;
use crate::parser::node::Statement;
use crate::parser::Parser;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::path::{MAIN_SEPARATOR, MAIN_SEPARATOR_STR};
use std::rc::Rc;

pub struct NextToByteCodeCompiler {
    cached_imports: Vec<Import>,
    paths: Vec<String>,
}

impl NextToByteCodeCompiler {
    pub fn new() -> Self {
        NextToByteCodeCompiler {
            cached_imports: vec![],
            paths: vec!["/usr/next/libs/".to_string(), "".to_string()],
        }
    }
}
pub fn compile_program(
    this: Rc<RefCell<NextToByteCodeCompiler>>,
    program: Vec<Statement>,
) -> Rc<RefCell<Context>> {
    let mut info_collector_visitor = InfoCollectorVisitor::new(Rc::clone(&this));
    info_collector_visitor.visit_program(program.clone());
    let info_ctx = info_collector_visitor.get_top_level_ctx();

    //let compiler_visitor = CompilerVisitor::new(Rc::new(info_ctx));

    //compiler_visitor.visit_program(program.clone());

    //let compiled_ctx = compiler_visitor.get_top_level_ctx();

    return info_ctx; //compiled_ctx;
}
pub fn importModule(this: Rc<RefCell<NextToByteCodeCompiler>>, name: String) -> Import {
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

    let path = fs::canonicalize(name.replace('.', MAIN_SEPARATOR_STR) + ".next")
        .expect(&*format!("The module {} not found", name))
        .as_path()
        .to_str()
        .unwrap()
        .to_string();

    for cached_import in &this.borrow().cached_imports {
        if cached_import.path == path {
            println!("Cached import");
            return cached_import.clone();
        }
    }

    let file_contents =
        fs::read_to_string(path.clone()).expect("Should have been able to read the file");

    let input = file_contents;

    let mut lexer = Lexer::new(
        input.chars().collect(),
        keywords.clone(),
        operators.clone(),
        specials.clone(),
    );
    lexer.read_char();

    let mut parser = Parser::new(
        lexer,
        operators_priorities.clone(),
        binary_operators.clone(),
        unary_operators.clone(),
    );

    let program = parser.parse();

    //todo!("Make fake import structure to avoid recursive death!");
    let idx = this.borrow().cached_imports.len();

    this.borrow_mut().cached_imports.push(Import {
        name: name.clone(),
        context: Rc::new(RefCell::new(Context {
            locals: Default::default(),
            functions: FunctionTable { functions: vec![] },
            types: TypeTable { types: vec![] },
            parent: None,
            imports: ImportTable { imports: vec![] },
            exports: ExportTable { exports: vec![] },
            uses: UsesTable { uses: vec![] },
        })),
        path: path.clone(),
    });

    let mut info_collector_visitor = InfoCollectorVisitor::new(Rc::clone(&this));
    info_collector_visitor.visit_program(program.clone());
    let info_ctx = info_collector_visitor.get_top_level_ctx();

    this.borrow_mut()
        .cached_imports
        .get_mut(idx)
        .unwrap()
        .context = Rc::clone(&info_ctx);

    return Import {
        name,
        context: info_ctx,
        path,
    };
}
