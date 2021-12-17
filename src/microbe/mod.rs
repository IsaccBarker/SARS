use crate::world::World;
use crate::genome::Genome;

use std::fmt;

use rand::rngs::ThreadRng;
use serde::{Serialize, Deserialize};

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
        let mut rng = rand::thread_rng();

        self.randomize_genome(&mut rng);

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

    pub fn mitos(self: &mut Self, rng: &mut ThreadRng) -> Self {
        let mut ret = Microbe::new();


        ret.genome = self.genome.mitos(500, rng);

        ret
    }

    pub fn tick(self: &mut Self) {
        let _rng = rand::thread_rng();

        /* if self.position.x < 0 {
            self.position.x = 0;
        }

        if self.position.y < 0 {
            self.position.y = 0;
        } */
    }
}

impl Microbe {
    fn randomize_genome(self: &mut Self, rng: &mut ThreadRng) {
        self.genome.randomize(rng);
    }
}

