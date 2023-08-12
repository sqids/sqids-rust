use sqids::*;

#[test]
fn simple() {
	let sqids = Sqids::default();

	let numbers = vec![1, 2, 3];
	let id = "8QRLaD";

	assert_eq!(sqids.encode(&numbers).unwrap(), id);
	assert_eq!(sqids.decode(id), numbers);
}

#[test]
fn different_inputs() {
	let sqids = Sqids::default();

	let numbers = vec![0, 0, 0, 1, 2, 3, 100, 1_000, 100_000, 1_000_000, Sqids::max_value()];

	assert_eq!(sqids.decode(&sqids.encode(&numbers).unwrap()), numbers);
}

#[test]
fn incremental_numbers() {
	let sqids = Sqids::default();

	let ids = vec![
		("bV", vec![0]),
		("U9", vec![1]),
		("g8", vec![2]),
		("Ez", vec![3]),
		("V8", vec![4]),
		("ul", vec![5]),
		("O3", vec![6]),
		("AF", vec![7]),
		("ph", vec![8]),
		("n8", vec![9]),
	];

	for (id, numbers) in ids {
		assert_eq!(sqids.encode(&numbers).unwrap(), id);
		assert_eq!(sqids.decode(id), numbers);
	}
}

#[test]
fn incremental_numbers_same_index_0() {
	let sqids = Sqids::default();

	let ids = vec![
		("SrIu", vec![0, 0]),
		("nZqE", vec![0, 1]),
		("tJyf", vec![0, 2]),
		("e86S", vec![0, 3]),
		("rtC7", vec![0, 4]),
		("sQ8R", vec![0, 5]),
		("uz2n", vec![0, 6]),
		("7Td9", vec![0, 7]),
		("3nWE", vec![0, 8]),
		("mIxM", vec![0, 9]),
	];

	for (id, numbers) in ids {
		assert_eq!(sqids.encode(&numbers).unwrap(), id);
		assert_eq!(sqids.decode(id), numbers);
	}
}

#[test]
fn incremental_numbers_same_index_1() {
	let sqids = Sqids::default();

	let ids = vec![
		("SrIu", vec![0, 0]),
		("nbqh", vec![1, 0]),
		("t4yj", vec![2, 0]),
		("eQ6L", vec![3, 0]),
		("r4Cc", vec![4, 0]),
		("sL82", vec![5, 0]),
		("uo2f", vec![6, 0]),
		("7Zdq", vec![7, 0]),
		("36Wf", vec![8, 0]),
		("m4xT", vec![9, 0]),
	];

	for (id, numbers) in ids {
		assert_eq!(sqids.encode(&numbers).unwrap(), id);
		assert_eq!(sqids.decode(id), numbers);
	}
}

#[test]
fn multi_input() {
	let sqids = Sqids::default();

	let numbers: Vec<u64> = (0..100).collect();
	let output = sqids.decode(&sqids.encode(&numbers).unwrap());

	assert_eq!(numbers, output);
}

#[test]
fn encoding_no_numbers() {
	let sqids = Sqids::default();
	assert_eq!(sqids.encode(&[]).unwrap(), "");
}

#[test]
fn decoding_empty_string() {
	let sqids = Sqids::default();
	let numbers: Vec<u64> = vec![];
	assert_eq!(sqids.decode(""), numbers);
}

#[test]
fn decoding_invalid_character() {
	let sqids = Sqids::default();
	let numbers: Vec<u64> = vec![];
	assert_eq!(sqids.decode("*"), numbers);
}

#[test]
fn decoding_invalid_id_with_repeating_reserved_character() {
	let sqids = Sqids::default();
	let numbers: Vec<u64> = vec![];
	assert_eq!(sqids.decode("fff"), numbers);
}
