use crate::taxonomy;

#[derive(Debug)]
pub struct Genus {
    /// The designation string.
    pub designation: String,

    /// Species under this genus.
    pub children: Vec<usize>,
}

impl Genus {
    pub fn new() -> Self {
        Self {
            designation: taxonomy::random_base_word() + "ae",
            children: vec![],
        }
    }
}

