use crate::genome::chromosome::BasePair;

use std::collections::HashMap;
use std::fmt;
use rand::rngs::ThreadRng;

pub fn random_resistant_genes(rng: &ThreadRng) -> HashMap<String, Gene> {
    let genes = HashMap::new();

    genes
}

pub fn random_neuronal_genes(rng: &ThreadRng) -> HashMap<String, Gene> {
    let genes = HashMap::new();

    genes
}

pub fn random_external_genes(rng: &mut ThreadRng) -> HashMap<String, Gene> {
    let mut genes: HashMap<String, Gene> = HashMap::new();

    // Base 4 colors. A=1, C=2, G=3, T=4
    // 300 (base eight) = 255 (base 10). Each digit comes from a base pair.
    // If the resulting number is more than 255, it will either clamp it or
    // wrap it back around, depending on the EC-COLW-1 gene.

    // Red color
    genes.insert("EC-COLR-R".to_owned(), Gene::from_existing(BasePair::random_base_pairs(3, rng)));
    // Green color
    genes.insert("EC-COLR-G".to_owned(), Gene::from_existing(BasePair::random_base_pairs(3, rng)));
    // Blue color
    genes.insert("EC-COLR-B".to_owned(), Gene::from_existing(BasePair::random_base_pairs(3, rng)));
    // Should we wrap around a color that is too big?
    genes.insert("EC-COLW-1".to_owned(), Gene::from_existing(vec![BasePair::random_base_pair(rng)]));

    // The higher the speed, the faster the microbe can move, but
    // motabalism increases.

    // Forward
    genes.insert("EC-SPD-F".to_owned(), Gene::from_existing(BasePair::random_base_pairs(2, rng)));

    // Backward
    genes.insert("EC-SPD-B".to_owned(), Gene::from_existing(BasePair::random_base_pairs(2, rng)));

    // Side
    genes.insert("EC-SPD-S".to_owned(), Gene::from_existing(BasePair::random_base_pairs(2, rng)));



    genes
}


/// A gene, located inside a chromosome.
/// *Generally*, the numbers that each pair represent are added to
/// get an output (pairs * 16):
///     (G, C) = (2, 1) = 2 + 1 = 3 (base 10, base four, base eight)
///     (T, G) = (4, 3) = 4 + 3 = 7 (base 10) = 4 (base eight) = 13 (base four)
///     (T, T) = (4, 4) = 4 + 4 = 8 (base 10) = 10 (base eight) = 20 (base eight)
/// The output of each base pair is combined into a gene output in a
/// specfic way, documented individually.
#[derive(Debug)]
pub struct Gene {
    pub pairs: Vec<BasePair>,
}

impl fmt::Display for Gene {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for pair in &self.pairs {
            write!(f, "{}", pair)?;
        }

        write!(f, "  ")
    }
}

impl Gene {
    pub fn new() -> Self {
        Self {
            pairs: vec![],
        }
    }

    pub fn from_existing(pairs: Vec<BasePair>) -> Self {
        Self {
            pairs,
        }
    }
}

