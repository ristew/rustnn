extern crate nalgebra;

use self::nalgebra::DVec;
use std::num::Float;
use std::collections::VecMap;

pub struct Neuron {
	bias: f64,
	weights: VecMap<f64>,
	input: f64,
	pub connections: Vec<usize>
}

impl Neuron {
	pub fn accept_input(&mut self, val: f64, conn: usize) {
		self.input = self.input + val * self.weights[conn];
	}
	pub fn fire (&self) -> f64 {
		sigmoid(self.bias + self.input)
	}
	pub fn add_connection(&mut self, conn: usize) {
		self.connections.push(conn);
		self.weights.insert(conn, 1.0);
	}
}

pub fn new () -> Neuron { 
		Neuron { bias: 0.0, weights: VecMap::new(), input: 0.0, connections: Vec::new() } 
}

fn sigmoid (v: f64) -> f64 {
	1.0 / (1.0 + Float::exp(-v))
}
