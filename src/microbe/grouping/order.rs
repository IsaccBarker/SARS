use crate::taxonomy;

#[derive(Debug)]
pub struct Order {
    /// The designation string.
    pub designation: String,

    /// Families under this order.
    pub children: Vec<usize>,
}

impl Order {
    pub fn new() -> Self {
        Self {
            designation: taxonomy::random_base_word() + "ales",
            children: vec![],
        }
    }
}

