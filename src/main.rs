pub mod world;
pub mod microbe;
pub mod genome;
pub mod position;

use world::World;

use indicatif::{ProgressBar, ProgressStyle};
use std::io::Write;

fn main() {
    let ticks: i32 = 100;

    let mut world = World::new(1000, 1000);
    let pb = ProgressBar::new(0);

    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}, {msg}] [{wide_bar:.cyan/blue}] ({eta})")
        .progress_chars("#>-"));

    pb.println("Creating microbes. This could take a while...");
    pb.println("Please note that this simulator does not cache state to a non-volatile medium. As such, all microbes are stored in memory.");
    pb.println("You may want a computer with a large amount of RAM to run a large scale simulation.");
    world.populate_microbes(pb, 10000000);

    let pb = ProgressBar::new(ticks as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}, {msg}] [{wide_bar:.cyan/blue}] ({eta})")
        .progress_chars("#>-"));

    world.tick();

    // println!("{}", world.microbes.get(0).unwrap());

    // I would much rather have this be parallel, but since
    // you would need to lock the world and the random
    // number generator, they would end up executing in
    // sequence anyways.
    for i in 0..ticks {
        world.tick();

        pb.set_position(i.try_into().unwrap());
        pb.set_message(format!("{}/{}", i, ticks));

        // pb.println(format!("{}, {}", world.microbes.get(0).unwrap().position.x, world.microbes.get(0).unwrap().orientation));
    }

     pb.finish_with_message("done");

     let mut json = std::fs::File::create("world.json").unwrap();
     json.write_all(serde_json::to_string(&world).unwrap().as_bytes());
}
