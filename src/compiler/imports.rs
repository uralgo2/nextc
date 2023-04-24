use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug, PartialEq)]
pub struct Import {
    name: String,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ImportTable {
    pub(crate) imports: Vec<Import>,
}

impl ImportTable {
    pub(crate) fn add(&mut self, import: String) -> Result<(), &str> {
        let import_name = Import {
            name: import.clone(),
        };
        if self.imports.contains(&import_name) {
            return Err("The same name already imported");
        }

        self.imports.push(import_name);

        Ok(())
    }
}
