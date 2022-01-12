pub mod grouping;

use crate::genome::Genome;
use crate::world::World;

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Microbe {
    pub genome: Genome,
}

impl fmt::Display for Microbe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Genome:\n{}", self.genome)
    }
}

impl Microbe {
    pub fn new() -> Self {
        Self {
            genome: Genome::new(),
        }
    }

    pub fn randomize(self: &mut Self, _world: &mut World) -> &mut Self {
        self.randomize_genome();

        let _genome = &self.genome;

        /* let mut speed: i32 = 0;
        let speed_gene = genome.external_chromosome.genes.get_key_value("EG-SPD-A").expect("EG-SPD-A gene not present!").1;

        speed += speed_gene.pairs.get(0).unwrap().a.value() as i32;
        speed += (speed_gene.pairs.get(0).unwrap().b.value() * 4) as i32;
        speed += (speed_gene.pairs.get(1).unwrap().a.value() * 8) as i32;
        speed += (speed_gene.pairs.get(1).unwrap().b.value() * 16) as i32;

        self.speed = speed; */

        self
    }

    pub fn mitos(self: &mut Self) -> Self {
        let mut ret = Microbe::new();

        ret.genome = self.genome.mitos(500);

        ret
    }

    pub fn tick(self: &mut Self) {}
}

impl Microbe {
    fn randomize_genome(self: &mut Self) {
        self.genome.randomize();
    }
}
