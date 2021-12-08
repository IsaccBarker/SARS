use crate::microbe::Microbe;

use indicatif::ProgressBar;
use serde::{Serialize, Deserialize};
use rayon::prelude::*;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub microbes: Vec<Microbe>,
    pub x_bound: i128,
    pub y_bound: i128,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

impl World {
    pub fn new(x_bound: i128, y_bound: i128) -> Self {
        Self {
            microbes: vec![],
            x_bound,
            y_bound,
        }
    }

    pub fn populate_microbes(self: &mut Self, pb: &ProgressBar, count: u32) {
        let mut i: u32 = 0;
        let mut microbe;

        pb.set_length(count.into());

        while i < count {
            microbe = Microbe::new();
            
            microbe.randomize(self);
            self.microbes.push(microbe);

            pb.inc(1);
            pb.set_message(format!("instance {}/{}", i, count));

            i = i + 1;
        }
    }

    #[allow(unused_variables, unused_mut)]
    pub fn tick(self: &mut Self) {
        let microbes = &mut self.microbes;

        microbes.into_par_iter().for_each(|microbe| {
            microbe.tick();
        });

        /* for microbe in microbes.into_par_iter() {
            microbe.tick(rng);
        } */
    }
}

