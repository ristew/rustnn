use std::collections::VecMap;
use std::rand::Rng;
use std::rand;
use node;

pub struct Net {
	nodes: Vec<node::Node>,
	node_connections: VecMap<VecMap<f64>>
}

impl Net {
	pub fn print(&self) {
		for i in range (0, self.nodes.len()) {
			println!("node: {}", i);
			for j in range (0, self.node_connections[i as usize].len()) {
				if i != j {
					println!("connection: {} weight: {}", j, self.node_connections[i as usize][j as usize]);
					self.nodes[i as usize].print();
				}
			}
		}
	}
	pub fn fire(&mut self) -> f64 {
		let mut tot: f64 = 0.0;
		for i in range (0, self.nodes.len()) {
			self.nodes[i].input = 0.0;
			for j in range (0, self.nodes.len()) {
				if i != j {
					let inp = self.nodes[j as usize].output * self.node_connections[i][j];
					self.nodes[i as usize].accept_input(inp);
				}
			}
		}
		for i in range (0, self.nodes.len()) {
			let o = self.nodes[i as usize].fire();
			self.nodes[i as usize].output = o;
			tot = tot + o;
		}
		tot
	}
	pub fn input_data(&mut self, data: Vec<f64>) {
		for i in range (0, self.nodes.len()) {
			self.nodes[i as usize].input_default = data[i as usize];
		}
	}
	pub fn train (&mut self, val: f64, data: Vec<f64>) {
		let dif = self.fire() - val;
		let mut o = 0.0;
		for i in range (0, self.nodes.len()) {
			self.nodes[i as usize].output = data[i as usize];
			o = self.nodes[i as usize].train(dif);
			for j in range (0, self.node_connections[i as usize].len()) {
				self.node_connections[i as usize][j as usize] -= o * 0.1;
			}
		}
	}
}

pub fn new_net(size: u32, nsize: u32) -> Net {
	let mut nds: Vec<node::Node> = Vec::new();
	let mut cts: VecMap<VecMap<f64>> = VecMap::new();
	let mut rng = rand::thread_rng();
	for i in range (0, size) {
		nds.push(node::new_node(nsize));
		cts.insert(i as usize, VecMap::new() );
		for j in range (0, size) {
			{
				cts[i as usize].insert(j as usize, 2.0 * rng.next_f64() - 1.0 );
			}
		}
	}
	Net { nodes: nds, node_connections: cts }
}
