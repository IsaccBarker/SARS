use crate::position::Position;
use crate::world::World;
use crate::genome::Genome;

use std::fmt;

use rand::prelude::*;
use rand::rngs::ThreadRng;
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Microbe {
    pub orientation: i32,
    pub speed: i32,
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
            orientation: 0,
            speed: 0,
            position: Position::new(0, 0),
            genome: Genome::new(),
        }
    }

    pub fn randomize(self: &mut Self, world: &mut World) -> &mut Self {
        let mut rng = rand::thread_rng();

        self.randomize_position(&mut rng, world.x_bound, world.y_bound);
        self.randomize_genome(&mut rng);

        let genome = &self.genome;

        let mut speed: i32 = 0;
        let speed_gene = genome.external_chromosome.genes.get_key_value("EG-SPD-A").expect("EG-SPD-A gene not present!").1;
        
        speed += speed_gene.pairs.get(0).unwrap().a.value() as i32;
        speed += (speed_gene.pairs.get(0).unwrap().b.value() * 4) as i32;
        speed += (speed_gene.pairs.get(1).unwrap().a.value() * 8) as i32;
        speed += (speed_gene.pairs.get(1).unwrap().b.value() * 16) as i32;

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

    pub fn tick(self: &mut Self) {
        let mut rng = rand::thread_rng();

        self.orientation += rng.gen_range(-1..2);
        self.position.x += self.speed as u32 * f64::cos(self.orientation as f64) as u32;
        self.position.y += self.speed as u32 * f64::sin(self.orientation as f64) as u32;

        /* if self.position.x < 0 {
            self.position.x = 0;
        }

        if self.position.y < 0 {
            self.position.y = 0;
        } */
    }
}

impl Microbe {
    fn randomize_position(self: &mut Self, rng: &mut ThreadRng, x_bound: u32, y_bound: u32) {
        self.position.x = rng.gen_range(1..x_bound);
        self.position.y = rng.gen_range(1..y_bound);
    }

    fn randomize_genome(self: &mut Self, rng: &mut ThreadRng) {
        self.genome.randomize(rng);
    }
}

