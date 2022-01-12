use super::Group;

use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Debug, Serialize, Deserialize)]
pub struct Strain {
    designation: String,

    parent: usize,
}

impl Group<'_> for Strain {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_random_standard_name(self: &mut Self) {
        // Get the parent using the usize index
        todo!("see comment");
        // self.designation = name;
    }

    fn get_standard_children(self: &Self) -> &Vec<usize> {
        panic!("this is the developers fault.");
    }
}
