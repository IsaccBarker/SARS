use crate::microbe::Microbe;
use crate::microbe::grouping::Group;

use rayon::prelude::*;
use indicatif::ProgressBar;
use serde::{Serialize, Deserialize};

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct World<'a> {
    #[serde(with = "serde_traitobject")]
    pub groups: Vec<Box<dyn Group<'a>>>,

    pub microbes: Vec<Microbe>,
    pub cached_microbes: i32,
}

unsafe impl<'a> Send for World<'a> {}
unsafe impl<'a> Sync for World<'a> {}

impl<'a> World<'a> {
    pub fn new() -> Self {
        Self {
            groups: vec![],
            microbes: vec![],
            cached_microbes: 0,
        }
    }

    pub fn populate_microbes(self: &mut Self, pb: &ProgressBar, count: u32) {
        let mut i: u32 = 0;
        let mut microbes: Vec<Microbe> = vec![];

        pb.set_length(count.into());

        while i < count {
            microbes.push(self.populate_microbe());
            pb.inc(1);
            pb.set_message(format!("instance {}/{}", i, count));

            i += 1;
        }

        self.microbes.append(&mut microbes);
    }

    fn populate_microbe(self: &mut Self) -> Microbe {
        let mut microbe;

        microbe = Microbe::new(); 
        microbe.randomize(self);

        microbe

    }

    #[allow(unused_variables, unused_mut)]
    pub fn tick(self: &mut Self) {
        let microbes = &mut self.microbes;

        microbes.par_iter_mut().for_each(|microbe| {
            microbe.tick();
        });

        // println!("{}", microbes.get(0).unwrap());

        /* let microbes = &mut self.microbes;

        microbes.into_par_iter().for_each(|microbe| {
            microbe.tick();
        }); */

        /* for microbe in microbes.into_par_iter() {
            microbe.tick(rng);
        } */
    }
}

