//! minmax volume f32, struct of arrays

use rand::{Rng, SeedableRng, rngs::StdRng};


const N: usize = 50_000;


fn main() {
	let mut rng = StdRng::seed_from_u64(42);
	let mut points = Points::new();
	for _ in 0 .. N {
		let x = rng.random_range(-100. .. 100.);
		let y = rng.random_range(-100. .. 100.);
		let z = rng.random_range(-100. .. 100.);
		points.push(x, y, z);
	}

	let mut volume_min: f32 = f32::MAX;
	let mut volume_max: f32 = f32::MIN;
	for i in 0 .. points.len() {
		for j in i+1 .. points.len() {
			let volume = points.volume_with(i, j);
			if volume < volume_min { volume_min = volume }
			if volume > volume_max { volume_max = volume }
		}
	}
	assert_eq!(0., volume_min);
	assert_eq!(7335174.5, volume_max);
	println!("volume min: {volume_min}");
	println!("volume max: {volume_max}");
}



#[derive(Debug, Clone)]
struct Points {
	x: Vec<f32>,
	y: Vec<f32>,
	z: Vec<f32>,
}
impl Points {
	fn volume_with(&self, i: u32, j: u32) -> f32 {
		let dx = (self.x[i as usize] - self.x[j as usize]).abs();
		let dy = (self.y[i as usize] - self.y[j as usize]).abs();
		let dz = (self.z[i as usize] - self.z[j as usize]).abs();
		dx * dy * dz
	}
	fn new() -> Self {
		Self { x: vec![], y: vec![], z: vec![] }
	}
	fn len(&self) -> u32 {
		self.x.len() as u32
	}
	fn push(&mut self, x: f32, y: f32, z: f32) {
		self.x.push(x);
		self.y.push(y);
		self.z.push(z);
	}
}

