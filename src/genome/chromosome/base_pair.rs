/// Self explanitory.
#[derive(Debug)]
pub enum DeoxyribonucleicAcid {
    A,
    C,
    T,
    G,
}

pub fn pairs_to_base_pairs(pairs: Vec<(DeoxyribonucleicAcid, DeoxyribonucleicAcid)>) -> Vec<BasePair> {
    let mut ret: Vec<BasePair> = Vec::new();

    for pair in pairs {
        ret.push(BasePair::from_existing(pair));
    }

    ret
}

/// Self explanitory.
#[derive(Debug)]
pub struct BasePair {
    pub a: DeoxyribonucleicAcid,
    pub b: DeoxyribonucleicAcid,
}

impl BasePair {
    pub fn new() -> Self {
        Self {
            a: DeoxyribonucleicAcid::A,
            b: DeoxyribonucleicAcid::C,
        }
    }

    pub fn from_existing(pair: (DeoxyribonucleicAcid, DeoxyribonucleicAcid)) -> Self {
        Self {
            a: pair.0,
            b: pair.1,
        }
    }
}

