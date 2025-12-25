//! primes



pub fn is_prime(n: u32) -> bool {
	if n == 0 { return false }
	if n == 1 { return false }
	if n == 2 { return true }
	if n % 2 == 0 { return false }
	for i in (3..=n.isqrt()+1).step_by(2) {
		if n % i == 0 {
			return false
		}
	}
	true
}



#[cfg(test)]
mod is_prime {
	use super::*;
	#[test] fn _0() { assert!(!is_prime(0)) }
	#[test] fn _1() { assert!(!is_prime(1)) }
	#[test] fn _2() { assert!(is_prime(2)) }
	#[test] fn _3() { assert!(is_prime(3)) }
	#[test] fn _4() { assert!(!is_prime(4)) }
	#[test] fn _5() { assert!(is_prime(5)) }
	#[test] fn _6() { assert!(!is_prime(6)) }
	#[test] fn _7() { assert!(is_prime(7)) }
	#[test] fn _8() { assert!(!is_prime(8)) }
	#[test] fn _9() { assert!(!is_prime(9)) }
	#[test] fn _10() { assert!(!is_prime(10)) }
	#[test] fn _11() { assert!(is_prime(11)) }
	#[test] fn _12() { assert!(!is_prime(12)) }
	#[test] fn _13() { assert!(is_prime(13)) }
	#[test] fn _14() { assert!(!is_prime(14)) }
	#[test] fn _15() { assert!(!is_prime(15)) }
	#[test] fn _16() { assert!(!is_prime(16)) }
	#[test] fn _17() { assert!(is_prime(17)) }
	#[test] fn _18() { assert!(!is_prime(18)) }
	#[test] fn _19() { assert!(is_prime(19)) }
	#[test] fn _20() { assert!(!is_prime(20)) }
	#[test] fn _21() { assert!(!is_prime(21)) }
	#[test] fn _22() { assert!(!is_prime(22)) }
	#[test] fn _23() { assert!(is_prime(23)) }
	#[test] fn _24() { assert!(!is_prime(24)) }
	#[test] fn _25() { assert!(!is_prime(25)) }
	#[test] fn _26() { assert!(!is_prime(26)) }
	#[test] fn _27() { assert!(!is_prime(27)) }
	#[test] fn _28() { assert!(!is_prime(28)) }
	#[test] fn _29() { assert!(is_prime(29)) }
	#[test] fn _30() { assert!(!is_prime(30)) }
	#[test] fn _31() { assert!(is_prime(31)) }
	#[test] fn _32() { assert!(!is_prime(32)) }
	#[test] fn _33() { assert!(!is_prime(33)) }
	#[test] fn _34() { assert!(!is_prime(34)) }
	#[test] fn _35() { assert!(!is_prime(35)) }
	#[test] fn _36() { assert!(!is_prime(36)) }
	#[test] fn _37() { assert!(is_prime(37)) }
	#[test] fn _38() { assert!(!is_prime(38)) }
	#[test] fn _39() { assert!(!is_prime(39)) }
	#[test] fn _40() { assert!(!is_prime(40)) }
	#[test] fn _41() { assert!(is_prime(41)) }
	#[test] fn _42() { assert!(!is_prime(42)) }
	#[test] fn _43() { assert!(is_prime(43)) }
	#[test] fn _44() { assert!(!is_prime(44)) }
	#[test] fn _45() { assert!(!is_prime(45)) }
	#[test] fn _46() { assert!(!is_prime(46)) }
	#[test] fn _47() { assert!(is_prime(47)) }
	#[test] fn _48() { assert!(!is_prime(48)) }
	#[test] fn _49() { assert!(!is_prime(49)) }
	#[test] fn _50() { assert!(!is_prime(50)) }
	#[test] fn _51() { assert!(!is_prime(51)) }
	#[test] fn _52() { assert!(!is_prime(52)) }
	#[test] fn _53() { assert!(is_prime(53)) }
	#[test] fn _54() { assert!(!is_prime(54)) }
	#[test] fn _55() { assert!(!is_prime(55)) }
	#[test] fn _56() { assert!(!is_prime(56)) }
	#[test] fn _57() { assert!(!is_prime(57)) }
	#[test] fn _58() { assert!(!is_prime(58)) }
	#[test] fn _59() { assert!(is_prime(59)) }
	#[test] fn _60() { assert!(!is_prime(60)) }
	#[test] fn _61() { assert!(is_prime(61)) }
	#[test] fn _62() { assert!(!is_prime(62)) }
	#[test] fn _63() { assert!(!is_prime(63)) }
	#[test] fn _64() { assert!(!is_prime(64)) }
	#[test] fn _65() { assert!(!is_prime(65)) }
	#[test] fn _66() { assert!(!is_prime(66)) }
	#[test] fn _67() { assert!(is_prime(67)) }
	#[test] fn _68() { assert!(!is_prime(68)) }
	#[test] fn _69() { assert!(!is_prime(69)) }
	#[test] fn _70() { assert!(!is_prime(70)) }
	#[test] fn _71() { assert!(is_prime(71)) }
	#[test] fn _72() { assert!(!is_prime(72)) }
	#[test] fn _73() { assert!(is_prime(73)) }
	#[test] fn _74() { assert!(!is_prime(74)) }
	#[test] fn _75() { assert!(!is_prime(75)) }
	#[test] fn _76() { assert!(!is_prime(76)) }
	#[test] fn _77() { assert!(!is_prime(77)) }
	#[test] fn _78() { assert!(!is_prime(78)) }
	#[test] fn _79() { assert!(is_prime(79)) }
	#[test] fn _80() { assert!(!is_prime(80)) }
	#[test] fn _81() { assert!(!is_prime(81)) }
	#[test] fn _82() { assert!(!is_prime(82)) }
	#[test] fn _83() { assert!(is_prime(83)) }
	#[test] fn _84() { assert!(!is_prime(84)) }
	#[test] fn _85() { assert!(!is_prime(85)) }
	#[test] fn _86() { assert!(!is_prime(86)) }
	#[test] fn _87() { assert!(!is_prime(87)) }
	#[test] fn _88() { assert!(!is_prime(88)) }
	#[test] fn _89() { assert!(is_prime(89)) }
	#[test] fn _90() { assert!(!is_prime(90)) }
	#[test] fn _91() { assert!(!is_prime(91)) }
	#[test] fn _92() { assert!(!is_prime(92)) }
	#[test] fn _93() { assert!(!is_prime(93)) }
	#[test] fn _94() { assert!(!is_prime(94)) }
	#[test] fn _95() { assert!(!is_prime(95)) }
	#[test] fn _96() { assert!(!is_prime(96)) }
	#[test] fn _97() { assert!(is_prime(97)) }
	#[test] fn _98() { assert!(!is_prime(98)) }
	#[test] fn _99() { assert!(!is_prime(99)) }
}

