use super::family::Family;

pub struct Order {
    /// The designation string.
    pub designation: String,

    /// Families under this order.
    pub classes: Vec<Family>,

}
