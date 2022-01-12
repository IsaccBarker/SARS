use super::Group;
use crate::taxonomy;

use std::any::Any;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Phylum {
    /// The designation string.
    pub designation: String,

    /// Classes under this phylum.
    pub classes: Vec<usize>,
}

impl Group<'_> for Phylum {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_random_standard_name(self: &mut Self) {
        self.designation = taxonomy::random_base_word() + "ylum";
    }

    fn get_standard_children(self: &Self) -> &Vec<usize> {
        &self.classes
    }
}

