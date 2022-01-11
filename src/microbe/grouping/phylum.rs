use super::class::Class;

pub struct Phylum {
    /// The designation string.
    pub designation: String,

    /// Classes under this phylum.
    pub classes: Vec<Class>,

}
