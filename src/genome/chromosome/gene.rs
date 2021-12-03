use crate::genome::chromosome::BasePair;
use crate::genome::chromosome::base_pair::{self, DeoxyribonucleicAcid::{self, A, C, G, T}};

use std::collections::HashMap;

pub fn random_resistant_genes() -> HashMap<String, Gene> {
    let genes = HashMap::new();

    genes
}

pub fn random_neuronal_genes() -> HashMap<String, Gene> {
    let genes = HashMap::new();

    genes
}

pub fn random_external_genes() -> HashMap<String, Gene> {
    let mut genes: HashMap<String, Gene> = HashMap::new();

    // Base 4 colors. A=0, C=1, G=2, T=3
    // The last acid does not matter.
    genes.insert("EC-COLR-R".to_owned(), Gene::from_existing(base_pair::pairs_to_base_pairs(vec![
                 (A, A), (A, A), (A, A), (A, A), (A, A)
    ])));

    genes.insert("EC-COLR-G".to_owned(), Gene::from_existing(base_pair::pairs_to_base_pairs(vec![
                 (A, A), (A, A), (A, A), (A, A), (A, A)
    ])));

    genes.insert("EC-COLR-B".to_owned(), Gene::from_existing(base_pair::pairs_to_base_pairs(vec![
                 (A, A), (A, A), (A, A), (A, A), (A, A)
    ])));


    genes
}


/// A gene, located inside a chromosome.
#[derive(Debug)]
pub struct Gene {
    pub pairs: Vec<BasePair>,
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

