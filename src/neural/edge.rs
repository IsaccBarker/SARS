use super::node::Node;

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Edge<'a> {
    pub to: Option<&'a mut Node<'a>>,
    pub weight: f64,
}

impl<'a> fmt::Display for Edge<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<'a> Edge<'a> {
    pub fn new(to: Option<&'a mut Node<'a>>, weight: f64) -> Self {
        Self {
            to,
            weight,
        }
    }
}

