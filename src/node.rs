use std::collections::VecMap;
use std::rand::Rng;
use std::rand;
use neuron;

pub struct Node {
	neurons: Vec<neuron::Neuron>,
	weights: VecMap<VecMap<f64>>,
	pub input: f64,
	pub input_default: f64,
	pub output: f64
}

impl Node {
	pub fn accept_input(&mut self, val: f64) {
		self.input = self.input + val;
	}
	pub fn print(&self) {
		for i in range (0, self.neurons.len()) {
			println!("node: {}", i);
			for j in range (0, self.weights[i as usize].len()) {
				if i != j {
					println!("neuron: {} weight: {}", j, self.weights[i as usize][j as usize]);
				}
			}
		}
	}
	pub fn input_data(&mut self, data: Vec<f64>) {
		for i in range (0, data.len()) {
			self.neurons[i as usize].input = (data[i as usize]);
		}
	}
	pub fn fire(&mut self) -> f64 {
		let mut tot: f64 = 0.0;
		for i in range (0, self.neurons.len()) {
			self.neurons[i].input = self.input_default;
			for j in range (0, self.neurons.len()) {
				if i != j {
					let inp = self.neurons[j as usize].output * self.weights[i][j];
					self.neurons[i as usize].accept_input(inp);
				}
			}
		}
		for i in range (0, self.neurons.len()) {
			let o = self.neurons[i as usize].fire();
			self.neurons[i as usize].output = o;
			tot = tot + o;
		}
		self.output = 0.0;
		tot
	}
	pub fn train(&mut self, val: f64) -> f64 {
		let mut tot = 0.0;
		for i in range (0, self.neurons.len()) {
			let w = self.neurons[i as usize].train(val);
			tot += w;
			for j in range (0, self.weights[i as usize].len()) {
				self.weights[i as usize][j as usize] -= 0.01 * w;
			}
		}
		tot
	}
}

pub fn new_node(size: u32) -> Node {
	let mut rng = rand::thread_rng();
	let mut nrs: Vec<neuron::Neuron> = Vec::new();
	let mut wts: VecMap<VecMap<f64>> = VecMap::new();
	for i in range (0, size) {
		wts.insert(i as usize, VecMap::new());
		for j in range (0, size) {
			wts[i as usize].insert(j as usize, 2.0 * rng.next_f64() - 1.0);
		}
		nrs.push(neuron::new_neuron());
	}
	Node { neurons: nrs, weights: wts, input: 0.0, input_default: 0.0, output: 0.0 }
}
