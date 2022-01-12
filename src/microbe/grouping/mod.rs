pub mod class;
pub mod family;
pub mod genus;
pub mod order;
pub mod phylum;
pub mod species;
pub mod strain;

use std::any::Any;

pub trait Group<'a>: std::fmt::Debug {
    fn as_any(&self) -> &dyn Any;
    fn set_random_standard_name(self: &mut Self);
    fn get_standard_children(&self) -> &Vec<usize>;
}
