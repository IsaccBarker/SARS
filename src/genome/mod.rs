pub mod chromosome;
pub mod ecdna;

use chromosome::Chromosome;
use chromosome::ChromosomeType;

use std::fmt;

/// Basically a nucleoid. While I know that prokaryotic organisms
/// really only have one chromosome, this makes it easier to represent
/// in coce. You can think of it as one anyways.
#[derive(Debug, PartialEq, Clone)]
pub struct Genome {
    pub general_chromosome: Chromosome,
}

impl fmt::Display for Genome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tGeneral: {}\n", self.general_chromosome)
    }
}

impl Genome {
    pub fn new() -> Self {
        Self {
            general_chromosome: Chromosome::new(),
        }
    }

    /// Randomizes the genome (chromosomes)
    pub fn randomize(self: &mut Self) {
        self.general_chromosome.randomize(ChromosomeType::General);
    }

    pub fn mitos(self: &mut Self, mutation_chance: i32) -> Self {
        let mut ret = Genome::new();

        ret.general_chromosome = self.general_chromosome.mitos(mutation_chance);
        // ret.external_chromosome = self.external_chromosome.mitos(mutation_chance);

        ret
    }

    pub fn get_in_full(self: &Self) -> &String {
        &self.general_chromosome.in_full
    }

    // Gets the string metric betwee two genomes with two-way.
    pub fn metric(self: &Self, other: String) -> f64 {
        // No idea how fast this is...
        strsim::normalized_damerau_levenshtein(self.get_in_full(), &other)
    }
}
