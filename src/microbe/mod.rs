use crate::position::Position;
use crate::world::World;
use crate::genome::Genome;

use std::fmt;
use rand::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Debug)]
pub struct Microbe {
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
            position: Position::new(0.0, 0.0),
            genome: Genome::new(),
        }
    }

    pub fn randomize(self: &mut Self, world: &mut World) -> &mut Self {
        self.randomize_position(&mut world.rng, world.x_bound, world.y_bound);
        self.randomize_genome(&mut world.rng);

        self
    }

    pub fn tick() {

    }
}

impl Microbe {
    fn randomize_position(self: &mut Self, rng: &mut ThreadRng, x_bound: f32, y_bound: f32) {
        self.position.x = rng.gen_range(-x_bound..x_bound);
        self.position.y = rng.gen_range(-y_bound..y_bound);
    }

    #[allow(unused_variables)]
    fn randomize_genome(self: &mut Self, rng: &mut ThreadRng) {
        self.genome.randomize(rng);
    }
}

