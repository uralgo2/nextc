use derivative::Derivative;
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Export {
    name: String,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct ExportTable {
    pub(crate) exports: Vec<Export>,
}
