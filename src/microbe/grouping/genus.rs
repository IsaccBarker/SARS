use super::species::Species;
use crate::taxonomy;

pub struct Genus {
    /// The designation string.
    pub designation: String,

    /// Species under this genus.
    pub classes: Vec<Species>,
}

impl Genus {
    pub fn random_genus_name() -> String {
        taxonomy::random_base_word() + "ae"
    }
}

