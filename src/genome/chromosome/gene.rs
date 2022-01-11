use crate::genome::chromosome::BasePair;
use crate::genome::chromosome::nucleobase::Nucleobase;

use std::collections::BTreeMap;
use std::fmt;
use std::iter::repeat_with;

use serde::{Serialize, Deserialize};

pub fn random_genes(short_designation: String) -> BTreeMap<String, Gene> {
    let mut genes = BTreeMap::new();
    // let en = Gene::from_existing(BasePair::random_base_pairs(500));

    for _ in 0..50 {
        let name: String = short_designation.to_owned() +
            &repeat_with(fastrand::alphanumeric).take(6).collect::<String>();

        genes.insert(name, Gene::from_existing(BasePair::random_base_pairs(50)));
    }

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub fn mitos(&self, mutation_rate: i32) -> Self {
        let mut ret = Gene::new();
        ret.pairs = self.pairs.clone();

        for pair in &mut ret.pairs {
            let x = fastrand::i32(0..10000);
            if x <= mutation_rate {
                if fastrand::i32(0..2) == 0 {
                    pair.a = Nucleobase::random_acid();

                    continue;
                }

                pair.b = Nucleobase::random_acid();
            }
        }
        
        ret
    }
}

