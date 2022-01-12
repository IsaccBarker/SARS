pub mod strain;
pub mod species;
pub mod genus;
pub mod family;
pub mod order;
pub mod class;
pub mod phylum;

use std::any::Any;

pub trait Group<'a>: std::fmt::Debug + serde_traitobject::Serialize + serde_traitobject::Deserialize {
    fn as_any(&self) -> &dyn Any;
    fn set_random_standard_name(self: &mut Self);
    fn get_standard_children(&self) -> &Vec<usize>;
}

// trait Group: std::fmt::Debug {}
// trait Group: serde::Serialize {}
// trait Group: serde::Deserialize {}

