use super::genus::Genus;

pub struct Family {
    /// The designation string.
    pub designation: String,

    /// Genuses under this family.
    pub classes: Vec<Genus>,
}

