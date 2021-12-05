pub mod node;
pub mod edge;

use node::{Node, NodeType};

use std::collections::HashMap;
use std::fmt;
use std::hash::BuildHasherDefault;

use rand::rngs::ThreadRng;
use rand::prelude::*;
use fxhash::FxHasher;

type HashMapFx<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

#[derive(Debug, PartialEq)]
pub struct NeuralNetwork<'a> {
    pub input: HashMapFx<String, Node<'a>>,
    pub hidden: Vec<Vec<Node<'a>>>,
    pub output: HashMapFx<String, Node<'a>>,
}

impl<'a> fmt::Display for NeuralNetwork<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tInput: {:?}\n\tHidden: {:?}\n\tOutput: {:?}", self.input, self.hidden, self.output)
    }
}

impl<'a> NeuralNetwork<'a> {
    pub fn new() -> Self {
        Self {
            input: HashMapFx::default(),
            hidden: vec![],
            output: HashMapFx::default(),
        }
    }

    pub fn set_input(self: &mut Self, name: String, value: f64) {
        if ! self.input.contains_key(&name) {
            self.input.insert(name, Node::new_simple(NodeType::Input, value));
            return;
        }

        (&mut *self.input.get_mut(&name).unwrap()).output = value;
    }

    pub fn set_inputs(self: &mut Self, pairs: Vec<(String, f64)>) {
        for pair in pairs {
            self.set_input(pair.0, pair.1);
        }
    }

    pub fn export_output(self: &mut Self, name: String) {
        if ! self.output.contains_key(&name) {
            self.output.insert(name, Node::new_simple(NodeType::Output, 0.0));
            return;
        }

        // (&mut *self.input.get_mut(&name).unwrap()).output = 0.0;
    }

    pub fn export_outputs(self: &mut Self, outputs: Vec<String>) {
        for output in outputs {
            self.export_output(output);
        }
    }

    pub fn randomize_edges_and_hidden(self: &mut Self, max_layer: i32, max_nodes: i32, rng: &mut ThreadRng) {
        let layers = rng.gen_range(0..max_layer);

        for l in (0..layers) {
            let nodes = rng.gen_range(0..max_nodes);

            for n in (0..nodes) {

            }
        }
    }

    pub fn process(self: &mut Self) {
        for (_, node) in &mut self.input {
            for edge in &mut node.edges {
                if edge.to.is_none() { continue; }

                edge.to.as_mut().unwrap().parameters.push(node.output * edge.weight);
            }

            node.process();
        }

        for layer in &mut self.hidden {
            for node in layer {
                for edge in &mut node.edges {
                    if edge.to.is_none() { continue; }

                    edge.to.as_mut().unwrap().parameters.push(node.output * edge.weight);
                }

                node.process();
            }
        }
    }
}

