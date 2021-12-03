use std::fmt;

use super::nucleobase::Nucleobase;

use rand::rngs::ThreadRng;

/// Self explanitory.
#[derive(Debug)]
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

    pub fn random_base_pair(rng: &mut ThreadRng) -> Self {
        BasePair::from_existing((
            Nucleobase::random_acid(rng),
            Nucleobase::random_acid(rng)))
    }

    pub fn random_base_pairs(pairs: i32, rng: &mut ThreadRng) -> Vec<BasePair> {
        let mut ret: Vec<BasePair> = vec![];

        for _ in 0..pairs {
            ret.push(BasePair::random_base_pair(rng));
        }

        ret
    }
}

