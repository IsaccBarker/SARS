pub mod base_pair;
pub mod gene;

use crate::genome::chromosome::base_pair::BasePair;
use crate::genome::chromosome::gene::Gene;

use std::collections::HashMap;

/// The type of chromosome.
/// 1. Resistant. Antibiotic resistance outside the cell.
/// 2. Neuronal. Neural network inside the cell.
/// 3. External. Physical characteristics outside the cell.
pub enum ChromosomeType {
    Resistant,
    Neuronal,
    External,
}

/// This is not a regular chromosome (e.g. it's not a sequence
/// of base pairs, but rather a description of the neural network
/// and characteristics of the microbe).
#[derive(Debug)]
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
    pub genes: HashMap<String, Gene>,
}

impl Chromosome {
    pub fn new(for_type: ChromosomeType) -> Self {
        let genes = match for_type {
            ChromosomeType::Resistant => gene::random_resistant_genes(),
            ChromosomeType::Neuronal => gene::random_neuronal_genes(),
            ChromosomeType::External => gene::random_external_genes(),
        };

        Self {
            genes,
        }
    }

    /// Performs mitosis. Takes the current chromosome, and
    /// spits out a version that underwent mitosis. Doesn't
    /// change the original chromosome.
    /// Mitosis is the process of replicating a chromosome.
    /// Sometimes, it goes wrong, and you get a genetic error,
    /// which is a key part of evolution.
    pub fn mitos(&self) -> Self {
        todo!();
    }
}

