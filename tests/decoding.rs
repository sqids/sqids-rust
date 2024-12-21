use sqids::*;

#[test]
fn decode_number_maximum_value() {
	let sqids = Sqids::default();
	let numbers = sqids.decode("ABARpJzdz9");
	assert_eq!(numbers, [9_007_199_254_740_991]); // 2 ^ 53
}

#[test]
fn decode_number_overflows() {
	let sqids = Sqids::default();
	let numbers = sqids.decode("0J4AEXRN106Z0");
	assert_eq!(numbers, Vec::<u64>::new());
}
