use sqids::*;
use std::collections::HashSet;

const UPPER: u64 = 1_000_000;

#[test]
fn uniques_with_padding() {
	let sqids = Sqids::new(Some(Options::new(None, Some(Options::default().alphabet.len()), None)))
		.unwrap();
	let mut set = HashSet::new();

	for i in 0..UPPER {
		let numbers = vec![i];
		let id = sqids.encode(&numbers).unwrap();
		set.insert(id.clone());
		assert_eq!(sqids.decode(&id), numbers);
	}

	assert_eq!(set.len(), UPPER as usize);
}

#[test]
fn uniques_low_ranges() {
	let sqids = Sqids::default();
	let mut set = HashSet::new();

	for i in 0..UPPER {
		let numbers = vec![i];
		let id = sqids.encode(&numbers).unwrap();
		set.insert(id.clone());
		assert_eq!(sqids.decode(&id), numbers);
	}

	assert_eq!(set.len(), UPPER as usize);
}

#[test]
fn uniques_high_ranges() {
	let sqids = Sqids::default();
	let mut set = HashSet::new();

	for i in 100_000_000..100_000_000 + UPPER {
		let numbers = vec![i];
		let id = sqids.encode(&numbers).unwrap();
		set.insert(id.clone());
		assert_eq!(sqids.decode(&id), numbers);
	}

	assert_eq!(set.len(), UPPER as usize);
}

#[test]
fn uniques_multi() {
	let sqids = Sqids::default();
	let mut set = HashSet::new();

	for i in 0..UPPER {
		let numbers = vec![i, i, i, i, i];
		let id = sqids.encode(&numbers).unwrap();
		set.insert(id.clone());
		assert_eq!(sqids.decode(&id), numbers);
	}

	assert_eq!(set.len(), UPPER as usize);
}
