use crate::taxonomy;
use super::Group;

use std::any::Any;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Genus {
    /// The designation string.
    pub designation: String,

    /// Species under this genus.
    pub classes: Vec<usize>,
}

impl Group<'_> for Genus {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_random_standard_name(self: &mut Self) {
        self.designation = taxonomy::random_base_word() + "ae";
    }

    fn get_standard_children(self: &Self) -> &Vec<usize> {
        &self.classes
    }
}

