pub mod base_pair;
pub mod gene;
pub mod nucleobase;

use crate::genome::chromosome::base_pair::BasePair;
use crate::genome::chromosome::gene::Gene;

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// The type of chromosome.
/// 1. General. Antibiotic resistance outside the cell.
/// 3. External. Physical characteristics outside the cell.
#[derive(Serialize, Deserialize)]
pub enum ChromosomeType {
    General,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Chromosome {
    pub genes: BTreeMap<String, Gene>,
}

impl fmt::Display for Chromosome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, v) in &self.genes {
            write!(f, "\n\t\t{}: {}", k, v)?;
        }

        write!(f, "")
    }
}

impl Chromosome {
    pub fn new() -> Self {
        Self {
            genes: BTreeMap::new(),
        }
    }

    pub fn randomize(&mut self, for_type: ChromosomeType) {
        self.genes = match for_type {
            ChromosomeType::General => gene::random_genes("".to_owned()),
        }
    }

    /// Performs mitosis. Takes the current chromosome, and
    /// spits out a version that underwent mitosis. Doesn't
    /// change the original chromosome.
    /// Mitosis is the process of replicating a chromosome.
    /// Sometimes, it goes wrong, and you get a genetic error,
    /// which is a key part of evolution.
    /// @param mutation_chance The percentage that a base pair
    ///     will be flipped for every gene that we iterate over.
    pub fn mitos(&self, mutation_chance: i32) -> Self {
        let mut ret = Chromosome::new();

        for gene in &self.genes {
            ret.genes
                .insert(gene.0.to_owned(), gene.1.mitos(mutation_chance));
        }

        ret
    }
}
