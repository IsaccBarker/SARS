use super::order::Order;
use crate::taxonomy;

pub struct Class {
    /// The designation string.
    pub designation: String,

    /// Orders under this class.
    pub classes: Vec<Order>,
}

impl Class {
    pub fn random_class_name() -> String {
        taxonomy::random_base_word() + "ia"
    }
}

