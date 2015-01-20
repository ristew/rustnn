use neuron;

pub struct Layer {
	neurons: Vec<neuron::Neuron>
}

impl Layer {
	pub fn new (&mut self, size: usize) -> Layer {
		let mut n: Vec<neuron::Neuron> = Vec::new();
		for i in range(0, size) {
			let o = neuron::new();
			n.push(o);
		}
		Layer { neurons: n }
	}
	pub fn update(&mut self) {
		for i in range(0, self.neurons.len()) {
			for j in range (0, self.neurons[i].connections.len()) {
				let val = self.neurons[self.neurons[i].connections[j]].fire();
				self.neurons[i].accept_input(val, j);
			}
		}
	}
}
