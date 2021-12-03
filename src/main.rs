pub mod world;
pub mod microbe;
pub mod genome;
pub mod position;

use world::World;

fn main() {
    let mut world = World::new(1000.0, 1000.0);

    world.populate_microbes(100);
}
