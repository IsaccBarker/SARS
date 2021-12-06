use crate::microbe::Microbe;

use rand::prelude::*;

#[derive(Debug)]
pub struct World {
    pub microbes: Vec<Microbe>,
    pub x_bound: f64,
    pub y_bound: f64,
    pub rng: rand::rngs::ThreadRng,
}

impl World {
    pub fn new(x_bound: f64, y_bound: f64) -> Self {
        Self {
            microbes: vec![],
            x_bound,
            y_bound,
            rng: thread_rng(),
        }
    }

    pub fn populate_microbes(self: &mut Self, count: u32) {
        let mut i: u32 = 0;
        let mut microbe;

        while i < count {
            microbe = Microbe::new();
            
            microbe.randomize(self);
            self.microbes.push(microbe);

            i = i + 1;
        }
    }

    #[allow(unused_variables, unused_mut)]
    pub fn tick(self: &mut Self, rng: &mut ThreadRng) {
        let microbes = &mut self.microbes;

        for microbe in microbes {
            microbe.tick(rng);
        }
    }
}

