extern crate rustnn;

fn main () {
	let mut net = rustnn::neuralnet::new_net(4, 8);
	println!("hi");
	net.print();
	let mut out: f64 = net.fire();
	println!("out: {}", out);
	let mut data: Vec<f64> = vec![0.5, 0.3, 0.0, 0.6];
	let newdata: Vec<f64> = vec![0.2, 0.0, 0.4, 0.5];
	for i in range (0, 500) {
		net.train(11.0, data.clone());
		net.train(4.0, newdata.clone());
	}
	net.input_data(data.clone());
	out = net.fire();
	net.input_data(newdata.clone());
	let newout = net.fire();
	net.print();
	println!("out: {}", out);
	println!("newout: {}", newout);
}

