use super::Group;
use crate::taxonomy;

use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Debug, Serialize, Deserialize)]
pub struct Class {
    /// The designation string.
    pub designation: String,

    /// Orders under this class.
    pub children: Vec<usize>,
}

impl Class {
    pub fn new() -> Self {
        Self {
            designation: "not designated".to_owned(),
            children: vec![],
        }
    }
}

impl Group<'_> for Class {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_random_standard_name(self: &mut Self) {
        self.designation = taxonomy::random_base_word() + "ia";
    }

    fn get_standard_children(self: &Self) -> &Vec<usize> {
        &self.children
    }
}
