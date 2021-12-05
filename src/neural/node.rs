use super::edge::Edge;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Input,
    Intermediate,
    Output,
}

#[derive(Debug, PartialEq)]
pub struct Node<'a> {
    pub node_type: NodeType,
    pub weight: Vec<f64>,
    pub bias: f64,
    pub parameters: Vec<f64>,
    pub output: f64,
    pub edges: Vec<Edge<'a>>,
}

impl<'a> fmt::Display for Node<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> Node<'a> {
    pub fn new(node_type: NodeType, weight: &[f64], bias: f64, output: f64) -> Self {
        Self {
            node_type,
            weight: weight.to_vec(),
            bias,
            parameters: vec![],
            output,
            edges: vec![],
        }
    }

    pub fn new_simple(node_type: NodeType, output: f64) -> Self {
        Self::new(node_type, &vec![], 0.0, output)
    }

    pub fn process(&mut self) {
        let mut sum: f64 = 0.0;

        for parameter in &self.parameters {
            sum += parameter;
        }

        sum += self.bias;

        self.output = sum.atan();
    }
}

