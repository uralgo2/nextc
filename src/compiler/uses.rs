use derivative::Derivative;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Using {
    name: String,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct UsesTable {
    pub(crate) uses: Vec<Using>,
}
