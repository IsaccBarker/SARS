use crate::microbe::Microbe;
use crate::microbe::grouping::*;

use indicatif::ProgressBar;
use rayon::prelude::*;

#[serde_with::serde_as]
#[derive(Debug)]
pub struct World {
    pub phylums: Vec<phylum::Phylum>,
    pub classes: Vec<class::Class>,
    pub orders: Vec<order::Order>,
    pub families: Vec<family::Family>,
    pub species: Vec<species::Species>,

    pub microbes: Vec<Microbe>,
    pub cached_microbes: i32,
}

unsafe impl Send for World {}
unsafe impl Sync for World {}

impl World {
    pub fn new() -> Self {
        let mut initial_phylum = phylum::Phylum::new();
        let mut initial_class = class::Class::new();
        let mut initial_order = order::Order::new();
        let mut initial_family = family::Family::new();
        let mut initial_genus = genus::Genus::new();
        let mut initial_species = species::Species::new(None);

        initial_phylum.children.push(0);
        initial_class.children.push(0);
        initial_order.children.push(0);
        initial_family.children.push(0);
        initial_genus.children.push(0);

        Self {
            phylums: vec![initial_phylum],
            classes: vec![initial_class],
            orders: vec![initial_order],
            families: vec![initial_family],
            species: vec![initial_species],

            microbes: vec![],
            cached_microbes: 0,
        }
    }

    pub fn populate_microbes(self: &mut Self, pb: &ProgressBar, count: u32) {
        let mut i: u32 = 0;
        let mut microbes: Vec<Microbe> = vec![];

        // I can't figure out how to bulk allocate this stuff.
        while i < count {
            microbes.push(self.new_random_microbe());
            pb.inc(1);
            pb.set_message(format!("instance {}/{}", i, count));

            i += 1;
        }

        self.microbes.append(&mut microbes);
    }

    pub fn classify_microbes(self: &mut Self, pb: &ProgressBar) {
        // Take the first microbe and put it in the existing taxonomical structure.
        let mut initial_species = self.species.get_mut(0).unwrap();
        let count = self.microbes.len();
        initial_species.reference_microbe = Some(0); // 0 is the index of the initial microbe.

        pb.inc(1);
        pb.set_message(format!("classified 1/{}", count));

        // Now classify the rest of the microbes.
        let i = 1;

        while i < count {
            let microbe = self.microbes.get(i).unwrap();

            // Compare the genome, from the top down in the taxonmical structure.
            

            pb.inc(1);
            pb.set_message(format!("classified {}/{}", i, count))
        }
    }

    fn new_random_microbe(self: &mut Self) -> Microbe {
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
    }
}
