//! minmax volume i32, struct of arrays

use rand::{Rng, SeedableRng, rngs::StdRng};


const N: usize = 50_000;


fn main() {
	let mut rng = StdRng::seed_from_u64(42);
	let mut points = Points::new();
	for _ in 0 .. N {
		let x = rng.random_range(-800 .. 800);
		let y = rng.random_range(-800 .. 800);
		let z = rng.random_range(-800 .. 800);
		points.push(x, y, z);
	}

	let mut volume_min: u32 = u32::MAX;
	let mut volume_max: u32 = u32::MIN;
	for i in 0 .. points.len() {
		for j in i+1 .. points.len() {
			let volume = points.volume_with(i, j);
			if volume < volume_min { volume_min = volume }
			if volume > volume_max { volume_max = volume }
		}
	}
	assert_eq!(2, volume_min);
	assert_eq!(3766837932, volume_max);
	println!("volume min: {volume_min}");
	println!("volume max: {volume_max}");
}



#[derive(Debug, Clone)]
struct Points {
	x: Vec<i32>,
	y: Vec<i32>,
	z: Vec<i32>,
}
impl Points {
	fn volume_with(&self, i: u32, j: u32) -> u32 {
		let dx = self.x[i as usize].abs_diff(self.x[j as usize]) + 1;
		let dy = self.y[i as usize].abs_diff(self.y[j as usize]) + 1;
		let dz = self.z[i as usize].abs_diff(self.z[j as usize]) + 1;
		dx * dy * dz
	}
	fn new() -> Self {
		Self { x: vec![], y: vec![], z: vec![] }
	}
	fn len(&self) -> u32 {
		self.x.len() as u32
	}
	fn push(&mut self, x: i32, y: i32, z: i32) {
		self.x.push(x);
		self.y.push(y);
		self.z.push(z);
	}
}

