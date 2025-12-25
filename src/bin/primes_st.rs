//! primes single thread

use benchmarks_by_dm::primes::is_prime;



fn main() {
	let mut total: u32 = 0;
	for n in 1 ..= 10_000_000 {
		if is_prime(n) {
			total += 1;
		}
	}
	println!("total: {total}");
}

