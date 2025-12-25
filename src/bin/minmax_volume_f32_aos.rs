//! minmax volume f32, array of structs

use rand::{Rng, SeedableRng, rngs::StdRng};


const N: usize = 50_000;


fn main() {
	let mut rng = StdRng::seed_from_u64(42);
	let mut points = Vec::with_capacity(N);
	for _ in 0 .. N {
		let x = rng.random_range(-100. .. 100.);
		let y = rng.random_range(-100. .. 100.);
		let z = rng.random_range(-100. .. 100.);
		points.push(Point { x, y, z });
	}

	let mut volume_min: f32 = f32::MAX;
	let mut volume_max: f32 = f32::MIN;
	for i in 0 .. points.len() {
		for j in i+1 .. points.len() {
			let volume = points[i].volume_with(points[j]);
			if volume < volume_min { volume_min = volume }
			if volume > volume_max { volume_max = volume }
		}
	}
	assert_eq!(0., volume_min);
	assert_eq!(7335174.5, volume_max);
	println!("volume min: {volume_min}");
	println!("volume max: {volume_max}");
}



#[derive(Debug, Clone, Copy)]
struct Point {
	x: f32,
	y: f32,
	z: f32,
}
impl Point {
	fn volume_with(self, other: Point) -> f32 {
		let dx = (self.x - other.x).abs();
		let dy = (self.y - other.y).abs();
		let dz = (self.z - other.z).abs();
		dx * dy * dz
	}
}

