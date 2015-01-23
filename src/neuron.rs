extern crate nalgebra;

use self::nalgebra::DVec;
use std::num::Float;
use std::collections::VecMap;

pub struct Neuron {
	bias: f64,
	pub input: f64,
	pub output: f64
}

impl Neuron {
	pub fn accept_input(&mut self, val: f64) {
		self.input = self.input + val;
	}
	pub fn fire (&self) -> f64 {
		sigmoid(self.bias + self.input)
	}
	pub fn train (&mut self, val: f64) -> f64 {
		let dif = deriv(val - self.fire());
		self.bias += 0.01 * dif;
		dif
	}
}

pub fn new_neuron () -> Neuron { 
	Neuron { bias: 0.0, input: 0.0, output: 0.0 } 
}

pub fn new_neuron_bias (b: f64) -> Neuron {
	Neuron { bias: b, input: 0.0, output: 0.0 }
}

pub fn sigmoid (v: f64) -> f64 {
	1.0 / (1.0 + Float::exp(-v))
}

pub fn deriv (v: f64) -> f64 {
	sigmoid(v) * (1.0 - sigmoid(v))
}
