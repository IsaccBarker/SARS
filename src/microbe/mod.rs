use crate::position::Position;
use crate::world::World;
use crate::genome::Genome;

use std::fmt;
use rand::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Debug, PartialEq)]
pub struct Microbe {
    pub orientation: f64,
    pub speed: f64,
    pub position: Position,
    pub genome: Genome,
}

impl fmt::Display for Microbe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position:\n\t{:?}\nGenome:\n{}", self.position, self.genome)
    }
}

impl Microbe {
    pub fn new() -> Self {
        Self {
            orientation: 0.0,
            speed: 0.0,
            position: Position::new(0.0, 0.0),
            genome: Genome::new(),
        }
    }

    pub fn randomize(self: &mut Self, world: &mut World) -> &mut Self {
        self.randomize_position(&mut world.rng, world.x_bound, world.y_bound);
        self.randomize_genome(&mut world.rng);

        let genome = &self.genome;

        let mut speed: f64 = 0.0;
        let speed_gene = genome.external_chromosome.genes.get_key_value("EG-SPD-A").expect("EG-SPD-A gene not present!").1;
        
        speed += speed_gene.pairs.get(0).unwrap().a.value() as f64;
        speed += (speed_gene.pairs.get(0).unwrap().b.value() * 4) as f64;
        speed += (speed_gene.pairs.get(1).unwrap().a.value() * 8) as f64;
        speed += (speed_gene.pairs.get(1).unwrap().b.value() * 16) as f64;

        self.speed = speed;

        self
    }

    pub fn mitos(self: &mut Self, rng: &mut ThreadRng) -> Self {
        let mut ret = Microbe::new();

        ret.position.x = self.position.x;
        ret.position.y = self.position.y;

        ret.genome = self.genome.mitos(500, rng);

        ret
    }

    pub fn tick(self: &mut Self, rng: &mut ThreadRng) {
        self.position.x += self.speed * self.orientation.cos();
        self.position.y += self.speed * self.orientation.sin();

        self.orientation = rng.gen_range(-10.0..10.0);
    }
}

impl Microbe {
    fn randomize_position(self: &mut Self, rng: &mut ThreadRng, x_bound: f64, y_bound: f64) {
        self.position.x = rng.gen_range(-x_bound..x_bound);
        self.position.y = rng.gen_range(-y_bound..y_bound);
    }

    fn randomize_genome(self: &mut Self, rng: &mut ThreadRng) {
        self.genome.randomize(rng);
    }
}

