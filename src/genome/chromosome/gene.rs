use crate::genome::chromosome::BasePair;
use crate::genome::chromosome::nucleobase::Nucleobase;

use std::collections::BTreeMap;
use std::fmt;
use rand::rngs::ThreadRng;
use rand::Rng;

pub fn random_resistant_genes(_rng: &ThreadRng) -> BTreeMap<String, Gene> {
    let genes = BTreeMap::new();

    genes
}

pub fn random_neuronal_genes(_rng: &ThreadRng) -> BTreeMap<String, Gene> {
    let genes = BTreeMap::new();

    // Inputs nodes.
    // genes.insert("NG-INPT-N");

    // Intermidate nodes.
    // genes.insert("NG-INTR-N");

    // Output nodes.
    // genes.insert("NG-OTPT-N");

    genes
}

pub fn random_external_genes(rng: &mut ThreadRng) -> BTreeMap<String, Gene> {
    let mut genes: BTreeMap<String, Gene> = BTreeMap::new();

    // Base 4 colors. A=1, C=2, G=3, T=4
    // 300 (base eight) = 255 (base 10). Each digit comes from a base pair.
    // If the resulting number is more than 255, it will either clamp it or
    // wrap it back around, depending on the EG-COLW-1 gene.

    // Red color
    genes.insert("EG-COLR-R".to_owned(), Gene::from_existing(BasePair::random_base_pairs(3, rng)));
    // Green color
    genes.insert("EG-COLR-G".to_owned(), Gene::from_existing(BasePair::random_base_pairs(3, rng)));
    // Blue color
    genes.insert("EG-COLR-B".to_owned(), Gene::from_existing(BasePair::random_base_pairs(3, rng)));
    // Should we wrap around a color that is too big?
    genes.insert("EG-COLW-1".to_owned(), Gene::from_existing(vec![BasePair::random_base_pair(rng)]));

    // The higher the speed, the faster the microbe can move, but
    // motabalism increases.

    // Forward
    genes.insert("EG-SPD-F".to_owned(), Gene::from_existing(BasePair::random_base_pairs(2, rng)));
    // Backward
    genes.insert("EG-SPD-B".to_owned(), Gene::from_existing(BasePair::random_base_pairs(2, rng)));
    // Side
    genes.insert("EG-SPD-S".to_owned(), Gene::from_existing(BasePair::random_base_pairs(2, rng)));

    // Capsule present
    genes.insert("EG-CAP-P".to_owned(), Gene::from_existing(vec![BasePair::random_base_pair(rng)]));

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
#[derive(Debug, Clone, PartialEq)]
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

    /// Makes the gene undergo mitosis. Non-destructive.
    /// @param mutation_rate The rate at which a base pair will
    ///     be mutated.
    pub fn mitos(&self, rng: &mut ThreadRng, mutation_rate: i32) -> Self {
        let mut ret = Gene::new();
        ret.pairs = self.pairs.clone();

        for pair in &mut ret.pairs {
            let x = rng.gen_range(0..10000);
            if x <= mutation_rate {
                if rng.gen_range(0..2) == 0 {
                    pair.a = Nucleobase::random_acid(rng);

                    continue;
                }

                pair.b = Nucleobase::random_acid(rng);
            }
        }
        
        ret
    }
}

