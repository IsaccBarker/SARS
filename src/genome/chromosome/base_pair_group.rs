use crate::genome::chromosome::base_pair::BasePair;

/// Has no biological equivilant, but it makes it easier
/// for the computer to look up related sequences.
pub struct BasePairGroup {
    pub label: String,
    pub pairs: Vec<BasePair>,
}

impl BasePairGroup {
    pub fn new(label: String, pairs: Vec<BasePair>) -> Self {
        Self {
            label,
            pairs,
        }
    }
}

