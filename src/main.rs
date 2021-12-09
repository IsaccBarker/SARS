pub mod world;
pub mod microbe;
pub mod genome;
pub mod position;

use world::World;

use indicatif::{ProgressBar, ProgressStyle};
use clap::{App, Arg};
use std::io::Write;

macro_rules! value_of_int_wrap {
    ($v:expr) => {
        $v.unwrap().parse().unwrap()
    }
}

fn main() {
    let opts = App::new("My Program")
        .author("Me, me@mail.com")
        .version("1.0.2")
        .about("Explains in brief what the program does")
        .arg(Arg::with_name("ticks").short("t").default_value("100000"))
        .arg(Arg::with_name("microbes").short("m").default_value("100000"))
        .arg(Arg::with_name("width").short("w").default_value("0.0"))
        .arg(Arg::with_name("height").short("h").default_value("0.0"))
        .arg(Arg::with_name("cache_every").short("c").default_value("250000"))
        .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
        .get_matches();

    let ticks: i32 = value_of_int_wrap!(opts.value_of("ticks"));
    let microbes: u32 = value_of_int_wrap!(opts.value_of("microbes"));
    let world_width: f64 = value_of_int_wrap!(opts.value_of("width"));
    let world_height: f64 = value_of_int_wrap!(opts.value_of("height"));
    let cache_every: u32 = value_of_int_wrap!(opts.value_of("cache_every"));

    let mut world = World::new(1, 1);
    let pb = ProgressBar::new(0);

    // pb.enable_steady_tick(0);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}, {msg}] [{wide_bar:.green/blue}] ({eta})")
        .progress_chars("#>."));

    pb.println("Creating microbes. This could take a while...");
    pb.println("Please note that this simulator does not cache state to a non-volatile medium. As such, all microbes are stored in memory.");
    pb.println("You may want a computer with a large amount of RAM to run a large scale simulation.");
    world.populate_microbes(&pb, microbes, cache_every);

    pb.reset();
    pb.reset_eta();
    pb.set_length(ticks as u64);

    world.tick();

    pb.println("Simulating...");

    // I would much rather have this be parallel, but since
    // you would need to lock the world and the random
    // number generator, they would end up executing in
    // sequence anyways.
    for i in 0..ticks {
        world.tick();

        pb.set_position(i.try_into().unwrap());
        pb.set_message(format!("epoch {}/{}", i, ticks));

        // pb.println(format!("{}, {}", world.microbes.get(0).unwrap().position.x, world.microbes.get(0).unwrap().orientation));
    }

     pb.set_message("serializing");

     let mut bc = std::fs::File::create("world.bc").unwrap();
     bc.write_all(&bincode::serialize(&world).unwrap()).expect("Failed to write serialized bincode world data!");

     pb.finish_with_message("done");
}
