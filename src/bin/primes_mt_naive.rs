//! primes multi thread

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use benchmarks_by_dm::primes::is_prime;



fn main() {
	let total: u32 = (1 .. 10_000_000_u32)
		.into_par_iter()
		.map(|n| {
			if is_prime(n) { 1 } else { 0 }
		})
		.sum();
	assert_eq!(664579, total);
	println!("total: {total}");
}

