//! minmax volume i32, array of structs

use rand::{Rng, SeedableRng, rngs::StdRng};


const N: usize = 50_000;


fn main() {
	let mut rng = StdRng::seed_from_u64(42);
	let mut points = Vec::with_capacity(N);
	for _ in 0 .. N {
		let x = rng.random_range(-800 .. 800);
		let y = rng.random_range(-800 .. 800);
		let z = rng.random_range(-800 .. 800);
		points.push(Point { x, y, z });
	}

	let mut volume_min: u32 = u32::MAX;
	let mut volume_max: u32 = u32::MIN;
	for i in 0 .. points.len() {
		for j in i+1 .. points.len() {
			let volume = points[i].volume_with(points[j]);
			if volume < volume_min { volume_min = volume }
			if volume > volume_max { volume_max = volume }
		}
	}
	assert_eq!(2, volume_min);
	assert_eq!(3766837932, volume_max);
	println!("volume min: {volume_min}");
	println!("volume max: {volume_max}");
}



#[derive(Debug, Clone, Copy)]
struct Point {
	x: i32,
	y: i32,
	z: i32,
}
impl Point {
	fn volume_with(self, other: Point) -> u32 {
		let dx = self.x.abs_diff(other.x) + 1;
		let dy = self.y.abs_diff(other.y) + 1;
		let dz = self.z.abs_diff(other.z) + 1;
		dx * dy * dz
	}
}

