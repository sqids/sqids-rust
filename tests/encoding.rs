use sqids::*;

#[test]
fn simple() {
	let sqids = Sqids::default();

	let numbers = vec![1, 2, 3];
	let id = "86Rf07";

	assert_eq!(sqids.encode(&numbers).unwrap(), id);
	assert_eq!(sqids.decode(id), numbers);
}

#[test]
fn different_inputs() {
	let sqids = Sqids::default();

	let numbers = vec![0, 0, 0, 1, 2, 3, 100, 1_000, 100_000, 1_000_000, u64::MAX];

	assert_eq!(sqids.decode(&sqids.encode(&numbers).unwrap()), numbers);
}

#[test]
fn incremental_numbers() {
	let sqids = Sqids::default();

	let ids = vec![
		("bM", vec![0]),
		("Uk", vec![1]),
		("gb", vec![2]),
		("Ef", vec![3]),
		("Vq", vec![4]),
		("uw", vec![5]),
		("OI", vec![6]),
		("AX", vec![7]),
		("p6", vec![8]),
		("nJ", vec![9]),
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
		("SvIz", vec![0, 0]),
		("n3qa", vec![0, 1]),
		("tryF", vec![0, 2]),
		("eg6q", vec![0, 3]),
		("rSCF", vec![0, 4]),
		("sR8x", vec![0, 5]),
		("uY2M", vec![0, 6]),
		("74dI", vec![0, 7]),
		("30WX", vec![0, 8]),
		("moxr", vec![0, 9]),
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
		("SvIz", vec![0, 0]),
		("nWqP", vec![1, 0]),
		("tSyw", vec![2, 0]),
		("eX68", vec![3, 0]),
		("rxCY", vec![4, 0]),
		("sV8a", vec![5, 0]),
		("uf2K", vec![6, 0]),
		("7Cdk", vec![7, 0]),
		("3aWP", vec![8, 0]),
		("m2xn", vec![9, 0]),
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
