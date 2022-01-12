use crate::taxonomy;
use super::Group;

use std::any::Any;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    /// The designation string.
    pub designation: String,

    /// Orders under this class.
    pub classes: Vec<usize>,
}

impl Group<'_> for Class {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_random_standard_name(self: &mut Self) {
        self.designation = taxonomy::random_base_word() + "ia";
    }

    fn get_standard_children(self: &Self) -> &Vec<usize> {
        &self.classes
    }
}

