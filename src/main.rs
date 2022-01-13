pub mod taxonomy;
pub mod world;
pub mod microbe;
pub mod genome;

use world::World;

use indicatif::{ProgressBar, ProgressStyle};
use clap::{App, Arg};

macro_rules! value_of_int_wrap {
    ($v:expr) => {
        $v.unwrap().parse().unwrap()
    }
}

#[allow(unreachable_code)]
fn main() {
    let opts = App::new("SARS")
        .author("Me, me@mail.com")
        .version("1.0.2")
        .about("Simple Anti-microbial Resistance Simulator. I am aware of the acronym's irony.")
        .arg(Arg::with_name("ticks").short("t").default_value("100000"))
        .arg(Arg::with_name("microbes").short("m").default_value("100000"))
        .arg(Arg::with_name("basepairs per gene").short("b").default_value("50"))
        .arg(Arg::with_name("genes per microbe").short("g").default_value("50"))
        .get_matches();

    let ticks: i32 = value_of_int_wrap!(opts.value_of("ticks"));
    let microbes: i32 = value_of_int_wrap!(opts.value_of("microbes"));
    // let basepairs: i32 = value_of_int_wrap!(opts.value_of("basepairs per gene"));
    // let genes: i32 = value_of_int_wrap!(opts.value_of("genes per microbe"));

    let mut world = World::new();
    let pb = ProgressBar::new(0);

    // pb.enable_steady_tick(0);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}, {msg}] [{wide_bar:.green/blue}] ({eta})")
        .progress_chars("#>."));

    pb.println("Initializing. This could take a while (linear time complexity)...");
    pb.println("You may want a computer with a large amount of RAM to run a large scale simulation.");
    pb.set_length(microbes.try_into().unwrap());
    world.populate_microbes(&pb, microbes.try_into().unwrap());

    pb.reset();
    pb.reset_eta();
    pb.set_length(microbes.try_into().unwrap());

    pb.println("\nClassifying. This could take a VERY long time (exponential time complexity)...");
    pb.println("You may want a computer with a lot of paralell processing capability (e.g. 32+ cores) to run a large scale simulation.");
    world.classify_microbes(&pb);

    pb.reset();
    pb.reset_eta();
    pb.set_length(ticks as u64);

    pb.println("\nSimulating. This could take a VERY long time (linear time complexity)...");
    pb.println("You may want a computer with a lot of paralell processing capability (e.g. 32+ cores) to run a large scale simulation.");


    for i in 0..ticks {
        world.tick();

        pb.set_position(i.try_into().unwrap());
        pb.set_message(format!("epoch {}/{}", i, ticks));
    }

     pb.finish_with_message("done");
}
