use crate::microbe::Microbe;

use rand::prelude::*;

#[derive(Debug)]
pub struct World<'a> {
    pub microbes: Vec<Microbe<'a>>,
    pub x_bound: f32,
    pub y_bound: f32,
    pub rng: rand::rngs::ThreadRng,
}

impl<'a> World<'a> {
    pub fn new(x_bound: f32, y_bound: f32) -> Self {
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
    pub fn tick(self: &mut Self, rng: &ThreadRng) {
        for microbe in &mut self.microbes {
            microbe.tick();
        }
    }
}

