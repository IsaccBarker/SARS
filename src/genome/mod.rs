pub mod ecdna;
pub mod chromosome;

use chromosome::Chromosome;
use chromosome::ChromosomeType;

use std::fmt;
use rand::rngs::ThreadRng;

/// Basically a nucleoid. While I know that prokaryotic organisms
/// really only have one chromosome, this makes it easier to represent
/// in coce. You can think of it as one anyways.
#[derive(Debug, PartialEq)]
pub struct Genome {
    pub resistant_chromosome: Chromosome,
    pub neuronal_chromosome: Chromosome,
    pub external_chromosome: Chromosome,
}

impl fmt::Display for Genome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tResistant: {}\n", self.resistant_chromosome)?;
        write!(f, "\tNeuronal : {}\n", self.neuronal_chromosome)?;
        write!(f, "\tExternal : {}\n", self.external_chromosome)
    }
}

impl Genome {
    pub fn new() -> Self {
        Self {
            resistant_chromosome: Chromosome::new(),
            neuronal_chromosome: Chromosome::new(),
            external_chromosome: Chromosome::new(),
        }
    }

    /// Randomizes the genome (chromosomes)
    pub fn randomize(self: &mut Self, rng: &mut ThreadRng) {
        self.resistant_chromosome.randomize(ChromosomeType::Resistant, rng);
        self.neuronal_chromosome.randomize(ChromosomeType::Neuronal, rng);
        self.external_chromosome.randomize(ChromosomeType::External, rng);
    }

    pub fn mitos(self: &mut Self, mutation_chance: i32, rng: &mut ThreadRng) -> Self {
        let mut ret = Genome::new();

        ret.resistant_chromosome = self.resistant_chromosome.mitos(mutation_chance, rng);
        ret.neuronal_chromosome = self.neuronal_chromosome.mitos(mutation_chance, rng);
        ret.external_chromosome = self.external_chromosome.mitos(mutation_chance, rng);

        ret
    }
}

