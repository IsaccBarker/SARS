use super::Group;
use crate::genome::Genome;
use crate::microbe::Microbe;
use crate::taxonomy;

use serde::{Deserialize, Serialize};
use std::any::Any;

#[derive(Debug, Serialize, Deserialize)]
pub struct Species {
    /// The genome that this species is references off of.
    pub reference: Genome,

    /// The designation string.
    pub designation: String,

    /// Strains under this species.
    pub strains: Vec<usize>,
}

impl Species {
    pub fn new() -> Self {
        Self {
            reference: Genome::new(),
            designation: "not designated".to_owned(),
            strains: vec![],
        }
    }

    pub fn from_microbe(microbe: &Microbe) -> Self {
        let mut s = Self::new();

        s.reference = microbe.genome.clone();

        s
    }
}

impl Group<'_> for Species {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn set_random_standard_name(self: &mut Self) {
        self.designation = taxonomy::random_base_word()
            .chars()
            .take(2)
            .collect::<String>()
            + &taxonomy::random_base_word()
                .chars()
                .take(2)
                .collect::<String>()
            + "-"
            + &fastrand::i32(0..100).to_string();
    }

    fn get_standard_children(self: &Self) -> &Vec<usize> {
        &self.strains
    }
}
