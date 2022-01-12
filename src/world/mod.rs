use crate::microbe::grouping::Group;
use crate::microbe::Microbe;
use crate::microbe::grouping::*;

use indicatif::ProgressBar;
use rayon::prelude::*;

#[serde_with::serde_as]
#[derive(Debug)]
pub struct World<'a> {
    pub groups: Vec<Box<dyn Group<'a>>>,
    pub microbes: Vec<Microbe>,
    pub cached_microbes: i32,
}

unsafe impl<'a> Send for World<'a> {}
unsafe impl<'a> Sync for World<'a> {}

impl<'a> World<'a> {
    pub fn new() -> Self {
        let mut groups: Vec<Box<dyn Group<'a>>> = vec![];
                                        
        let mut initial_phylum = Box::new(phylum::Phylum::new());
        let mut initial_class = Box::new(class::Class::new());
        let mut initial_order = Box::new(order::Order::new());
        let mut initial_family = Box::new(family::Family::new());
        let mut initial_genus = Box::new(genus::Genus::new());
        let mut initial_species = Box::new(species::Species::new());

        // initial_phylum.children.push(1);

        groups.append(&mut vec![
            initial_phylum,
            initial_class,
            initial_order,
            initial_family,
            initial_genus,
            initial_species,
        ]);

        groups.get_mut(0).unwrap().as_any().downcast_mut::<&mut phylum::Phylum>();
            // .expect("group[0] isn't a phylum").children.push(1);

        // groups.get_mut(0).unwrap().as_any().downcast_mut::<phylum::Phylum>();

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

        // I can't figure out how to bulk allocate this stuff.
        while i < count {
            microbes.push(self.new_random_microbe());
            pb.inc(1);
            pb.set_message(format!("instance {}/{}", i, count));

            i += 1;
        }

        self.microbes.append(&mut microbes);
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
