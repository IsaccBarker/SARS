use crate::position::Position;
use crate::world::World;
use crate::genome::Genome;
use crate::neural::NeuralNetwork;

use std::fmt;
use rand::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Debug, PartialEq)]
pub struct Microbe<'a> {
    pub position: Position,
    pub neural: NeuralNetwork<'a>,
    pub genome: Genome,
}

impl<'a> fmt::Display for Microbe<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Position:\n\t{:?}\nNeural:\n{}\nGenome:\n{}", self.position, self.neural, self.genome)
    }
}

impl<'a> Microbe<'a> {
    pub fn new() -> Self {
        let mut neural = NeuralNetwork::new();
        neural.export_outputs(vec![
            "speed".to_owned(),
            "direction".to_owned(),
        ]);

        Self {
            position: Position::new(0.0, 0.0),
            neural,
            genome: Genome::new(),
        }
    }

    pub fn randomize(self: &mut Self, world: &mut World) -> &mut Self {
        self.randomize_position(&mut world.rng, world.x_bound, world.y_bound);
        self.randomize_genome(&mut world.rng);

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
        self.neural.set_inputs(vec![
            ("x".to_owned(), self.position.x.into()),
            ("y".to_owned(), self.position.y.into()),
        ]);

        self.neural.process();
    }
}

impl<'a> Microbe<'a> {
    fn randomize_position(self: &mut Self, rng: &mut ThreadRng, x_bound: f32, y_bound: f32) {
        self.position.x = rng.gen_range(-x_bound..x_bound);
        self.position.y = rng.gen_range(-y_bound..y_bound);
    }

    fn randomize_genome(self: &mut Self, rng: &mut ThreadRng) {
        self.genome.randomize(rng);
    }
}

