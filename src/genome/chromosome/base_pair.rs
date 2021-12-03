use std::fmt;

use rand::Rng;
use rand::rngs::ThreadRng;

/// Self explanitory.
#[derive(Debug)]
pub enum Nucleobase {
    A,
    C,
    T,
    G,
    NYA,
}

impl fmt::Display for Nucleobase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Nucleobase {
    pub fn random_acid(rng: &mut ThreadRng) -> Nucleobase {
        match rng.gen_range(0..4) { // rand 0.8
            0 => Nucleobase::A,
            1 => Nucleobase::C,
            2 => Nucleobase::T,
            3 => Nucleobase::G,
            _ => unreachable!(),
        }
    }
}

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

    /* pub fn pairs_to_base_pairs(pairs: Vec<Self>) -> Vec<BasePair> {
        let mut ret: Vec<BasePair> = Vec::new();

        for pair in pairs {
            ret.push(BasePair::from_existing(pair));
        }

        ret
    } */
}

