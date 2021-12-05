pub mod world;
pub mod microbe;
pub mod genome;
pub mod neural;
pub mod position;

use world::World;

use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let ticks: i32 = 100000;

    let mut world = World::new(1000.0, 1000.0);
    let pb = ProgressBar::new(ticks.try_into().unwrap());
    let rng = rand::thread_rng();
    
    world.populate_microbes(1000);

    world.tick(&rng);

    println!("{}", world.microbes.get(0).unwrap());

    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}, {msg}] [{wide_bar:.cyan/blue}] ({eta})")
        .progress_chars("#>-"));

    // I would much rather have this be parallel, but since
    // you would need to lock the world and the random
    // number generator, they would end up executing in
    // sequence anyways.
    for i in 0..ticks {
        world.tick(&rng);

        pb.set_position(i.try_into().unwrap());
        pb.set_message(format!("{}/{}", i, ticks));
    }

     pb.finish_with_message("done");
}
