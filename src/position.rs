use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: i128,
    pub y: i128,
}

impl Position {
    pub fn new(x: i128, y: i128) -> Self {
        Self {
            x,
            y,
        }
    }
}

