use super::Group;
use crate::taxonomy;

use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Debug, Serialize, Deserialize)]
pub struct Family {
    /// The designation string.
    pub designation: String,

    /// Genuses under this family.
    pub children: Vec<usize>,
}

impl Family {
    pub fn new() -> Self {
        Self {
            designation: "not designated".to_owned(),
            children: vec![],
        }
    }
}

impl Group<'_> for Family {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_random_standard_name(self: &mut Self) {
        self.designation = taxonomy::random_base_word() + "aceae";
    }

    fn get_standard_children(self: &Self) -> &Vec<usize> {
        &self.children
    }
}
