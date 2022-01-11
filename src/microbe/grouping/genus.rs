use super::species::Species;

pub struct Genus {
    /// The designation string.
    pub designation: String,

    /// Species under this genus.
    pub classes: Vec<Species>,

}

