//! primes single thread

use benchmarks_by_dm::primes::is_prime;



fn main() {
	let mut total: u32 = 0;
	for n in 1 .. 10_000_000_u32 {
		if is_prime(n) {
			total += 1;
		}
	}
	assert_eq!(664579, total);
	println!("total: {total}");
}

