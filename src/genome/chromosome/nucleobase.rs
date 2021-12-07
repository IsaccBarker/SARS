use std::fmt;

use rand::rngs::ThreadRng;
use rand::prelude::*;
use serde::{Serialize, Deserialize};

/// Self explanitory.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

    pub fn value(&self) -> u8 {
        match self {
            Nucleobase::A => 0,
            Nucleobase::C => 1,
            Nucleobase::T => 2,
            Nucleobase::G => 3,
            _ => panic!("Unknown nucleobase {}!", self),
        }
    }
}
