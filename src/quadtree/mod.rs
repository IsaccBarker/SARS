use crate::microbe::Microbe;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quad {
    pub children: Vec<Microbe>,
}

impl Quad {
    pub fn new() -> Self {
        Self {
            children: vec![], 
        }
    }
}

