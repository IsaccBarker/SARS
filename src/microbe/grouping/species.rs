use crate::taxonomy;

#[derive(Debug)]
pub struct Species {
    /// The designation string.
    pub designation: String,

    pub reference_microbe: Option<usize>,
}

impl Species {
    pub fn new(reference: Option<usize>) -> Self {
        Self {
            designation: Self::get_random_standard_name(),
            reference_microbe: reference,
        }
    }

    fn get_random_standard_name() -> String {
        taxonomy::random_base_word()
            .chars()
            .take(2)
            .collect::<String>()
            + &taxonomy::random_base_word()
                .chars()
                .take(2)
                .collect::<String>()
            + "-"
            + &fastrand::i32(0..100).to_string()
    }
}

