pub mod class;
pub mod family;
pub mod genus;
pub mod order;
pub mod phylum;
pub mod species;

pub mod threshold {
    // Basically just guesswork.

    pub const NEW_PHYLUM_THRESHOLD: f64 = 0.05;
    pub const NEW_CLASS_THRESHOLD: f64 = 0.1;
    pub const NEW_ORDER_THRESHOLD: f64 = 0.2;
    pub const NEW_FAMILY_THRESHOLD: f64 = 0.25;
    pub const NEW_GENUS_THRESHOLD: f64 = 0.4;
    pub const NEW_SPECIES_THRESHOLD: f64 = 0.6;
}

