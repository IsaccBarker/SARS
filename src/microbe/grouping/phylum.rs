use crate::taxonomy;

#[derive(Debug)]
pub struct Phylum {
    /// The designation string.
    pub designation: String,

    /// Classes under this phylum.
    pub children: Vec<usize>,
}

impl Phylum {
    pub fn new() -> Self {
        Self {
            designation: taxonomy::random_base_word() + "ylum",
            children: vec![],
        }
    }
}

