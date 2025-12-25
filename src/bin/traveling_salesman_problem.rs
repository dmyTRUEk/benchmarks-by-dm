//! traveling salesman problem

use itertools::Itertools;
use rand::{Rng, SeedableRng, rngs::StdRng};


const N: usize = 11;


fn main() {
	let mut rng = StdRng::seed_from_u64(42);
	let mut points = Vec::with_capacity(N);
	for _ in 0 .. N {
		let x = rng.random_range(-100. .. 100.);
		let y = rng.random_range(-100. .. 100.);
		points.push(Point { x, y });
	}
	let points = points;

	let shortest_path_dist: f32 = points.iter().permutations(N)
		.map(|points| -> f32 {
			let mut dist = 0.;
			let mut p_prev = points[0];
			for p in points.iter().skip(1) {
				dist += p_prev.dist_to(p);
				p_prev = p;
			}
			dist
		})
		.min_by(|d1, d2| d1.partial_cmp(d2).unwrap())
		.unwrap()
	;
	assert_eq!(581.61237, shortest_path_dist);
	println!("shortest path distance: {shortest_path_dist}");
}



#[derive(Debug, Clone, Copy)]
struct Point { x: f32, y: f32 }
impl Point {
	fn dist_to(&self, other: &Self) -> f32 {
		((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
	}
}

