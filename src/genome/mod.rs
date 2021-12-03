pub mod ecdna;
pub mod chromosome;

use chromosome::Chromosome;

/// Basically a nucleoid. While I know that prokaryotic organisms
/// really only have one chromosome, this makes it easier to represent
/// in coce. You can think of it as one anyways.
#[derive(Debug)]
pub struct Genome {
    pub resistant_chromosome: Chromosome,
    pub neuronal_chromosome: Chromosome,
    pub external_chromosome: Chromosome,
}

impl Genome {
    /// Creates a new genome.
    pub fn new() -> Self {
        Self {
            resistant_chromosome: Chromosome::new(),
            neuronal_chromosome: Chromosome::new(),
            external_chromosome: Chromosome::new(),
        }
    }
}

