pub mod base_pair;
pub mod base_pair_group;

use std::collections::HashMap;

/// This is not a regular chromosome (e.g. it's not a sequence
/// of base pairs, but rather a description of the neural network
/// and characteristics of the microbe).
#[derive(Debug)]
pub struct Chromosome {
    pub pairs: HashMap<String, String>,
}

impl Chromosome {
    /// Creates a new chromosome.
    pub fn new() -> Self {
        Self {
            pairs: HashMap::new(),
        }
    }

    /// Performs mitosis. Takes the current chromosome, and
    /// spits out a version that underwent mitosis. Doesn't
    /// change the original chromosome.
    pub fn mitos(&self) -> Self {
        Self {
            pairs: self.pairs.clone(),
        }
    }
}

