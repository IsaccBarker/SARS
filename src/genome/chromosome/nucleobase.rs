use rand::rngs::ThreadRng;
use rand::prelude::*;

use std::fmt;

/// Self explanitory.
#[derive(Debug, Clone, PartialEq)]
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
