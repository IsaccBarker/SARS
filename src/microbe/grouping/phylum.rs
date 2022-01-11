use super::class::Class;
use crate::taxonomy;

pub struct Phylum {
    /// The designation string.
    pub designation: String,

    /// Classes under this phylum.
    pub classes: Vec<Class>,
}

impl Phylum {
    pub fn random_phylum_name() -> String {
        taxonomy::random_base_word() + "ylum"
    }
}

