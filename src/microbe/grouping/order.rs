use super::family::Family;
use crate::taxonomy;

pub struct Order {
    /// The designation string.
    pub designation: String,

    /// Families under this order.
    pub classes: Vec<Family>,
}

impl Order {
    pub fn random_order_name() -> String {
        taxonomy::random_base_word() + "ales"
    }
}

