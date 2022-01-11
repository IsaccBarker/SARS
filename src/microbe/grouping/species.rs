use super::strain::Strain;
use crate::genome::Genome;
use crate::microbe::Microbe;

pub struct Species {
    /// The genome that this species is references off of.
    pub reference: Genome,

    /// The designation string.
    pub designation: String,

    /// Strains under this species.
    pub strains: Vec<Strain>,
}

impl Species {
    pub fn new() -> Self {
        Self {
            reference: Genome::new(),
            designation: "not designated".to_owned(),
            strains: vec![],
        }
    }

    pub fn from_microbe(microbe: &Microbe) -> Self {
        let mut s = Self::new();

        s.reference = microbe.genome.clone();
    }
}

