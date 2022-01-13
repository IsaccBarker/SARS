use crate::taxonomy;

#[derive(Debug)]
pub struct Family {
    /// The designation string.
    pub designation: String,

    /// Genuses under this family.
    pub children: Vec<usize>,
}

impl Family {
    pub fn new() -> Self {
        Self {
            designation: taxonomy::random_base_word() + "aceae",
            children: vec![],
        }
    }
}

