use crate::microbe::Microbe;
use crate::quadtree::Quad;

use indicatif::ProgressBar;
use serde::{Serialize, Deserialize};

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    // pub microbes: Vec<Microbe>,
    pub quads: Vec<Quad>,
    pub n_quads: usize,
    pub cached_microbes: i32,
    pub x_bound: u32,
    pub y_bound: u32,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

impl World {
    pub fn new(x_bound: u32, y_bound: u32, n_quads: usize) -> Self {
        let mut quads = Vec::new();

        for _ in 0..(n_quads^2) {
            quads.push(Quad::new());
        }

        Self {
            quads,
            n_quads,
            cached_microbes: 0,
            x_bound,
            y_bound,
        }
    }

    pub fn populate_microbes(self: &mut Self, pb: &ProgressBar, count: u32) {
        let mut i: u32 = 0;
        // let mut microbes: Vec<Microbe> = vec![];

        pb.set_length(count.into());

        while i < count {
            let microbe = self.populate_microbe();

            for quad in &self.quads {
                // Detect the X position of the target quad.
                let x = self.n_quads as u32 / microbe.position.x;

                // Detect the Y position of the target quad.
            }

            // microbes.push(self.populate_microbe());
            
            pb.inc(1);
            pb.set_message(format!("instance {}/{}", i, count));

            i += 1;
        }
    }

    fn populate_microbe(self: &mut Self) -> Microbe {
        let mut microbe;

        microbe = Microbe::new(); 
        microbe.randomize(self);

        microbe

    }

    #[allow(unused_variables, unused_mut)]
    pub fn tick(self: &mut Self) {
        /* let microbes = &mut self.microbes;

        microbes.into_par_iter().for_each(|microbe| {
            microbe.tick();
        }); */

        /* for microbe in microbes.into_par_iter() {
            microbe.tick(rng);
        } */
    }
}

