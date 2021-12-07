pub mod nucleobase;
pub mod base_pair;
pub mod gene;

use crate::genome::chromosome::base_pair::BasePair;
use crate::genome::chromosome::gene::Gene;

use std::collections::BTreeMap;
use std::fmt;

use rand::rngs::ThreadRng;
use serde::{Serialize, Deserialize};

/// The type of chromosome.
/// 1. Resistant. Antibiotic resistance outside the cell.
/// 2. Neuronal. Neural network inside the cell.
/// 3. External. Physical characteristics outside the cell.
#[derive(Serialize, Deserialize)]
pub enum ChromosomeType {
    Resistant,
    Neuronal,
    External,
}

/// This is not a regular chromosome (e.g. it's not a sequence
/// of base pairs, but rather a description of the neural network
/// and characteristics of the microbe).
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Chromosome {
    /// The base pairs that belong to the chromosome.
    /// Each gene is labeled with the String key,
    /// whereas the vector of BasePairs is the
    /// representation itself.
    /// The name of the gene is determined as folows:
    ///     EG-COLR-R1
    /// E = External
    /// G = Gene
    /// - = Seperator
    /// COLR = Two/three letter ID used to denote which gene in
    ///     a human readble way.
    /// - = Seperator
    /// R1 = Two character unique identifier used if two gene
    ///     have the same ID.
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

    pub fn randomize(&mut self, for_type: ChromosomeType, rng: &mut ThreadRng) {
        self.genes = match for_type {
            ChromosomeType::Resistant => gene::random_resistant_genes(rng),
            ChromosomeType::Neuronal => gene::random_neuronal_genes(rng),
            ChromosomeType::External => gene::random_external_genes(rng),
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
    pub fn mitos(&self, mutation_chance: i32, rng: &mut ThreadRng) -> Self {
        let mut ret = Chromosome::new();

        for gene in &self.genes {
            ret.genes.insert(gene.0.to_owned(), gene.1.mitos(rng, mutation_chance));
        }

        ret
    }
}

