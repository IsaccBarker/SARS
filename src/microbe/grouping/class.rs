use super::order::Order;

pub struct Class {
    /// The designation string.
    pub designation: String,

    /// Orders under this class.
    pub classes: Vec<Order>,

}

