use crate::microbe::Microbe;
use crate::microbe::grouping::*;
use crate::microbe::grouping::threshold::*;
use crate::genome::Genome;

use indicatif::ProgressBar;
use rayon::prelude::*;

#[serde_with::serde_as]
#[derive(Debug)]
pub struct World {
    pub phylums: Vec<phylum::Phylum>,
    pub classes: Vec<class::Class>,
    pub orders: Vec<order::Order>,
    pub families: Vec<family::Family>,
    pub genuses: Vec<genus::Genus>,
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
        let initial_species = species::Species::new(None);

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
            genuses: vec![initial_genus],
            species: vec![initial_species],

            microbes: vec![],
            cached_microbes: 0,
        }
    }


    fn get_phylum_reference(self: &Self, i: usize) -> &Genome {
        self.get_class_reference(*self.phylums.get(i).unwrap().children.get(0).unwrap())
    }

    fn get_class_reference(self: &Self, i: usize) -> &Genome {
        self.get_order_reference(*self.classes.get(i).unwrap().children.get(0).unwrap())
    }

    fn get_order_reference(self: &Self, i: usize) -> &Genome {
        self.get_family_reference(*self.families.get(i).unwrap().children.get(0).unwrap())
    }

    fn get_family_reference(self: &Self, i: usize) -> &Genome {
        self.get_genus_reference(*self.genuses.get(i).unwrap().children.get(0).unwrap())
    }

    fn get_genus_reference(self: &Self, i: usize) -> &Genome {
        let microbe_index = self.species.get(i).unwrap().reference_microbe.unwrap();

        &self.microbes.get(microbe_index).unwrap().genome
    }


    fn new_from_phylum(self: &mut Self) -> usize {
        self.phylums.push(phylum::Phylum::new());
        self.phylums.len() - 1
    }

    fn new_from_class(self: &mut Self) -> usize {
        self.classes.push(class::Class::new());
        self.classes.len() - 1
    }

    fn new_from_order(self: &mut Self) -> usize {
        self.orders.push(order::Order::new());
        self.orders.len() - 1
    }

    fn new_from_family(self: &mut Self) -> usize {
        self.families.push(family::Family::new());
        self.families.len() - 1
    }

    fn new_from_genus(self: &mut Self) -> usize {
        self.genuses.push(genus::Genus::new());
        self.genuses.len() - 1
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
            let genome_in_full = microbe.genome.get_in_full();
            let mut max_phylum_metric = (0.0, 0);
            let mut max_class_metric = (0.0, 0);
            let mut max_order_metric = (0.0, 0);
            let mut max_family_metric = (0.0, 0);
            let mut max_genus_metric = (0.0, 0);
            let mut max_species_metric = (0.0, 0);

            // Compare the genome, from the top down in the taxonmical structure.
            for (i, _) in self.phylums.iter().enumerate() {
                let reference_genome = self.get_phylum_reference(i);
                let metric = reference_genome.metric(genome_in_full.to_owned());

                if metric > max_phylum_metric.0 { max_phylum_metric.1 = i; max_phylum_metric.0 = metric; }
            }

            if max_phylum_metric.0 < NEW_PHYLUM_THRESHOLD {}


            for (i, _) in self.classes.iter().enumerate() {
                let reference_genome = self.get_class_reference(i);
                let metric = reference_genome.metric(genome_in_full.to_owned());
            
                if metric > max_class_metric.0 { max_class_metric.1 = i; max_class_metric.0 = metric; }
            }

            if max_class_metric.0 < NEW_CLASS_THRESHOLD {}


            for (i, _) in self.orders.iter().enumerate() {
                let reference_genome = self.get_order_reference(i);
                let metric = reference_genome.metric(genome_in_full.to_owned());
            
                if metric > max_order_metric.0 { max_order_metric.1 = i; max_order_metric.0 = metric; }
            }

            if max_order_metric.0 < NEW_ORDER_THRESHOLD {}


            for (i, _) in self.families.iter().enumerate() {
                let reference_genome = self.get_family_reference(i);
                let metric = reference_genome.metric(genome_in_full.to_owned());
            
                if metric > max_family_metric.0 { max_family_metric.1 = i; max_family_metric.0 = metric; }
            }

            if max_family_metric.0 < NEW_FAMILY_THRESHOLD {}


            for (i, _) in self.genuses.iter().enumerate() {
                let reference_genome = self.get_genus_reference(i);
                let metric = reference_genome.metric(genome_in_full.to_owned());
            
                if metric > max_genus_metric.0 { max_genus_metric.1 = i; max_genus_metric.0 = metric; }
            }

            if max_genus_metric.0 < NEW_GENUS_THRESHOLD {}


            for (i, _) in self.species.iter().enumerate() {
                let reference_microbe_index = self.species.get(i).unwrap().reference_microbe.unwrap();
                let reference_genome = &self.microbes.get(reference_microbe_index).unwrap().genome;
                let metric = reference_genome.metric(genome_in_full.to_owned());
            
                if metric > max_species_metric.0 { max_species_metric.1 = i; max_species_metric.0 = metric; }
            }

            if max_species_metric.0 < NEW_SPECIES_THRESHOLD {}


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
