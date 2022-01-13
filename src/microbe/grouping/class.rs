use crate::taxonomy;

#[derive(Debug)]
pub struct Class {
    /// The designation string.
    pub designation: String,

    /// Orders under this class.
    pub children: Vec<usize>,
}

impl Class {
    pub fn new() -> Self {
        Self {
            designation:  taxonomy::random_base_word() + "ia",
            children: vec![],
        }
    }
}

