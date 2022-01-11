use std::fmt;

use super::nucleobase::Nucleobase;

use serde::{Serialize, Deserialize};

/// Self explanitory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BasePair {
    pub a: Nucleobase,
    pub b: Nucleobase,
}

impl fmt::Display for BasePair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.a, self.b)
    }
}

impl BasePair {
    pub fn new() -> Self {
        Self {
            a: Nucleobase::NYA,
            b: Nucleobase::NYA,
        }
    }

    pub fn from_existing(pair: (Nucleobase, Nucleobase)) -> Self {
        Self {
            a: pair.0,
            b: pair.1,
        }
    }

    pub fn random_base_pair() -> Self {
        BasePair::from_existing((
            Nucleobase::random_acid(),
            Nucleobase::random_acid()))
    }

    pub fn random_base_pairs(pairs: i32) -> Vec<BasePair> {
        let mut ret: Vec<BasePair> = vec![];

        for _ in 0..pairs {
            ret.push(BasePair::random_base_pair());
        }

        ret
    }
}

