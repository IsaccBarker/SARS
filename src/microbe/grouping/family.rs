use super::genus::Genus;
use crate::taxonomy;

pub struct Family {
    /// The designation string.
    pub designation: String,

    /// Genuses under this family.
    pub classes: Vec<Genus>,
}

impl Family {
    pub fn random_family_name() -> String {
        taxonomy::random_base_word() + "aceae"
    }
}

